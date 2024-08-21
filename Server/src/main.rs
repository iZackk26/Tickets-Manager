use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpListener;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::server::socket::handle_client;
use crate::stadium::structures::{Seat, Status, Zone};
use mpmcpq::{PriorityQueue, Stash, Message};
use crate::algorithm::get_best_seats;
use crate::server::Buyer::Buyer;

mod stadium;
mod algorithm;
mod server;

fn process_order(buyer: Buyer, stadium: &mut HashMap<String, Zone>){
    let seat: Vec<Seat> = get_best_seats(stadium, buyer.section_type, buyer.quantity as u8);

}

fn main() {
    let mut stadium: HashMap<String, Zone> = stadium::data::generate_stadium();
    //algorithm::get_best_seats(&mut stadium, "shaded".to_string(), 3);

    let priority_queue: Arc<PriorityQueue<Buyer, i8>> = Arc::new(PriorityQueue::new());
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on port 7878");


    // Hilo que procesa los datos de la PriorityQueue
    let pq = Arc::clone(&priority_queue);
    thread::spawn( move || {
        while let Message::Msg(buyer, priority)= pq.recv() {
            //println!("{:?}", buyer.conection.unwrap().write("HOLAAA".as_bytes()));
            process_order(buyer, &mut stadium);

        }
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let priority_queue = Arc::clone(&priority_queue);
                thread::spawn(move || {
                    if let Ok(buyer) = handle_client(stream) {
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
