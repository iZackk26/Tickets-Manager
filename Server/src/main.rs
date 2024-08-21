use std::collections::HashMap;
use std::net::TcpListener;
use std::thread;
use crate::server::socket::handle_client;
use crate::stadium::structures::{Status, Zone};
use mpmcpq::{PriorityQueue, Stash, Message};
use crate::server::Buyer::Buyer;

mod stadium;
mod algorithm;
mod server;


fn main() {
    let priority_queue: PriorityQueue<Buyer, i8> = PriorityQueue::new();
    let mut stash: Stash<Buyer, i8> = Stash::new(&priority_queue);

    /*
    let priority1 = buyer.Quantity.clone();
    priority_queue.send(buyer, -priority1, &mut stash);
     */


    while let Message::Msg(message, priority) = priority_queue.recv() {
        println!("Processing task: {:?} with priority: {}", message, priority);
    }
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
