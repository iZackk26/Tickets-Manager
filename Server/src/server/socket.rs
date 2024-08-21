use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write}; // Asegúrate de que `Read` está importado
use crate::server::Buyer::Buyer;

pub fn handle_client(mut stream: TcpStream){
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Conexión cerrada por el cliente
                println!("Client disconnected");
                break;
            }
            Ok(n) => {
                // Deserializar el JSON recibido
                let received_data = &buffer[0..n];
                match serde_json::from_slice::<Buyer>(received_data) {
                    Ok(message) => {
                        println!("Received message: {:?}", message);
                        // ...
                    }
                    Err(e) => {
                        eprintln!("Failed to deserialize JSON: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                break;
            }
        }
    }

}
