use std::collections::HashMap;
use std::net::TcpListener;
use std::thread;
use mpmcpq::{PriorityQueue};
use crate::server::socket::handle_client;
use crate::stadium::structures::{Zone};

mod stadium;
mod algorithm;
mod server;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Failed to accept the conection {}", e)
            }

        }
    }

    let mut stadium : HashMap<String, Zone> = stadium::data::generate_stadium();

    //println!("{:?}", stadium.get("north").unwrap().categories.get(&'a').unwrap().rows.get(&'w').unwrap().seats);
    //algorithm::test(&stadium);
}
