use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::server::socket::{parse_client};
use crate::stadium::structures::{Seat, Status, Zone};
use mpmcpq::{PriorityQueue, Stash, Message};
use crate::algorithm::{fill_stadium, get_best_seats, modify_seats_status};
use crate::server::buyer::Buyer;

mod stadium;
mod algorithm;
mod server;


fn main() {
    let mut stadium: HashMap<String, Zone> = stadium::data::generate_stadium();
    fill_stadium(&mut stadium, 0.91); // Se llena al 91 %
    //println!("{:?}", algorithm::get_best_seats(&mut stadium, &"shaded".to_string(), 3));

    let priority_queue: Arc<PriorityQueue<Buyer, i8>> = Arc::new(PriorityQueue::new());
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on port 8080");


    // Hilo que procesa los datos de la PriorityQueue
    let pq = Arc::clone(&priority_queue);
    thread::spawn( move || {
        loop {
            match pq.recv() {
                Message::Msg(buyer, priority) => {
                    let seats: Vec<Seat> = get_best_seats(&mut stadium, &buyer.section_type, buyer.quantity as u8);

                    let mut con = buyer.connection.unwrap();

                    if !seats.is_empty() {
                        match serde_json::to_string(&seats) {
                            Ok(seats) => {
                                con.write(&seats.as_bytes());
                            }
                            Err(e) => {
                                println!("{:?}", e)
                            }
                        }
                        // Wait for the client response

                        // Create a buffer to read the client response
                        let mut buffer = [0;512];
                        let n = con.read(&mut buffer).expect("Error reading the client response");
                        let response_data = &buffer[0..n];
                        // Here we recieve the response from the client that is if he will buy the seats or not
                        //let client_response: serde_json::Value = serde_json::from_slice(response_data).expect("Error parsing the client response");
                        let client_response: serde_json::Value = match serde_json::from_slice(response_data) {
                            Ok(value) => value,
                            Err(e) => {
                                println!("Error al parsear la respuesta del cliente: {:?}", e);
                                modify_seats_status(&mut stadium, seats, Status::Available);
                                // Aquí puedes decidir cómo manejar el error, por ejemplo, cerrar la conexión o enviar un mensaje de error
                                match con.write(b"Respuesta invalida recibida. Cerrando conexion.") {
                                    Ok(_) => println!("Mensaje de error enviado al cliente."),
                                    Err(e) => println!("Error al enviar mensaje de error al cliente: {:?}", e),
                                };
                                con.shutdown(Shutdown::Both).unwrap_or_else(|e| {
                                    println!("Error al cerrar la conexion: {:?}", e);
                                });
                                return;
                            }
                        };

                        // Check if the client response is true or false, the default value is false
                        if client_response["response"].as_bool().unwrap_or(false) {
                            println!("Client accepted the seats");
                            // ...
                            modify_seats_status(&mut stadium, seats, Status::Purchased);
                            // Send a message to the client to close the connection
                            match con.write(b"Purchase completed, closing connection") {
                                // If the confirmation was sent successfully
                                Ok(_) => println!("Confirmation sent to the client."),
                                Err(e) => println!("Error sending the confirmation to client: {:?}", e),
                            }
                        } else {
                            println!("Client rejected the seats");
                            modify_seats_status(&mut stadium, seats, Status::Available);
                            // Send a message to the client to close the connection
                            match con.write(b"Seats were rejected, closing connection") {
                                // If the confirmation was sent successfully
                                Ok(_) => println!("Confirmacion enviada al cliente."),
                                Err(e) => println!("Error sending the confirmation to client: {:?}", e),
                            }
                        }

                    } else {
                        con.write(b"No hay asientos disponibles");
                    }

                    match con.shutdown(Shutdown::Both) {
                        Ok(_) => println!("Conexión cerrada correctamente"),
                        Err(e) => println!("Error al cerrar la conexión: {:?}", e),
                    }
                }

                Message::Drained => {
                    println!("Drained")
                }
                Message::Taken => {
                    println!("Taken")
                }
            }
        }
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Peticion Recibida {:?}", stream.peer_addr());
                let priority_queue = Arc::clone(&priority_queue);
                thread::spawn(move || {
                    if let Ok(buyer) = parse_client(stream) {
                        println!("Print 1er hilo");
                        let priority: i8 = -buyer.quantity.clone();
                        priority_queue.send_nostash(buyer, priority);
                    }
                });
            }
            Err(e) => {
                println!("Failed to accept the conection {}", e)
            }

        }
    }
}