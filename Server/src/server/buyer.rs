use serde::{Deserialize, Serialize};
use std::net::TcpStream;


#[derive(Serialize, Deserialize, Debug)]
pub struct Buyer {
    pub section_type: String,
    pub quantity: i8,
    #[serde(skip_serializing, skip_deserializing)]
    pub connection: Option<TcpStream>
}