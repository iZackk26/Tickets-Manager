use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;
use std::net::{Shutdown, TcpListener};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::server::socket::handle_client;
use crate::stadium::structures::{Seat, Status, Zone};
use mpmcpq::{PriorityQueue, Stash, Message};
use crate::algorithm::get_best_seats;
use crate::server::buyer::Buyer;

mod stadium;
mod algorithm;
mod server;


fn main() {
    let mut stadium: HashMap<String, Zone> = stadium::data::generate_stadium();
    //algorithm::get_best_seats(&mut stadium, "shaded".to_string(), 3);

    let priority_queue: Arc<PriorityQueue<Buyer, i8>> = Arc::new(PriorityQueue::new());
    let listener = TcpListener::bind("192.168.0.104:8080").unwrap();
    println!("Server listening on port 7878");



    // Hilo que procesa los datos de la PriorityQueue
    let pq = Arc::clone(&priority_queue);
    thread::spawn( move || {
        loop {
            match pq.recv() {
                Message::Msg(buyer, priority) => {
                    let seats: Vec<Seat> = get_best_seats(&mut stadium, &buyer.section_type, buyer.quantity as u8);
                    let mut con = buyer.conection.unwrap();
                    //con.write(serde_json::to_string(&seats).unwrap().as_bytes());

                    match serde_json::to_string(&seats) {
                        Ok(seats) => {
                            con.write(&seats.as_bytes());
                        }
                        Err(e) => {
                            println!("{:?}", e)
                        }
                    }
                    con.shutdown(Shutdown::Both).unwrap();
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
                    if let Ok(buyer) = handle_client(stream) {
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