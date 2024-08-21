use serde::{Deserialize, Serialize};
use std::net::TcpStream;

#[derive(Serialize, Deserialize, Debug)]
pub struct Buyer {
    pub section_type: String,
    pub quantity: i8,
    pub response_time: u32,
    pub response: bool,
    // Never serialized.
    #[serde(skip_serializing, skip_deserializing)]
    pub conection: Option<TcpStream>
}