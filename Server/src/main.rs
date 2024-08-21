use std::cell::RefCell;
use std::collections::HashMap;
use std::net::TcpListener;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::server::socket::handle_client;
use crate::stadium::structures::{Status, Zone};
use mpmcpq::{PriorityQueue, Stash, Message};
use crate::server::Buyer::Buyer;

mod stadium;
mod algorithm;
mod server;


fn main() {
    let priority_queue: Arc<PriorityQueue<Buyer, i8>> = Arc::new(PriorityQueue::new());

    // while let Message::Msg(message, priority) = priority_queue.recv() {
    //     println!("Processing task: {:?} with priority: {}", message, priority);
    // }

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let priority_queue = Arc::clone(&priority_queue);
                thread::spawn(move || {
                    if let Ok(buyer) = handle_client(stream) {
                        let priority: i8 = buyer.Quantity.clone();
                        priority_queue.send_nostash(buyer, priority);
                    }
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
