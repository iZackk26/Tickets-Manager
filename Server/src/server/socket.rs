
use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write}; // Asegúrate de que `Read` está importado
use crate::server::buyer::Buyer;


pub fn parse_client(mut stream: TcpStream) -> Result<(Buyer), Box<dyn std::error::Error>> {
    let mut buffer = [0; 512];

    // Leer la primera parte: section_type y quantity
    let n = stream.read(&mut buffer)?;
    let received_data = &buffer[0..n];
    let partial_buyer: serde_json::Value = serde_json::from_slice(received_data)?;

    // Extraer los valores
    let section_type = partial_buyer["section_type"].as_str().unwrap_or("").to_string();
    let quantity = partial_buyer["quantity"].as_i64().unwrap_or(0) as i32;
    let buyer = Buyer {
        section_type : section_type,
        quantity: quantity as i8,
        connection: Option::Some(stream)
    };

    Ok(buyer)
}
