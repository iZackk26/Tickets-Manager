
use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write}; // Asegúrate de que `Read` está importado
use crate::server::buyer::Buyer;

pub fn handle_client(mut stream: TcpStream) -> Result<Buyer, Box<dyn std::error::Error>>{
    let mut buffer = [0; 512];
        match stream.read(&mut buffer) {
            Ok(n) => {
                // Deserializar el JSON recibido
                let received_data = &buffer[0..n];
                let mut buyer = serde_json::from_slice::<Buyer>(received_data)?;
                buyer.conection = Some(stream);
                Ok(buyer)
                //Ok(serde_json::from_slice::<Buyer>(received_data)?)
            }
            Err(e) => {
                Err(e.into())
            }
        }

}
