use crate::algorithm::{fill_stadium, get_best_seats, modify_seats_status};
use crate::server::buyer::Buyer;
use crate::server::socket::parse_client;
use crate::stadium::structures::{Seat, Status, Zone};
use mpmcpq::{Message, PriorityQueue, Stash};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;

mod algorithm;
mod server;
mod stadium;

fn main() {
    let mut stadium: HashMap<String, Zone> = stadium::data::generate_stadium();
    fill_stadium(&mut stadium, 0.5);

    let section_type = "shaded".to_string();
    let seats = algorithm::get_best_seats(&mut stadium, &section_type, 3);
    for seat in &seats {
        println!("{:?}", seat);
    }
    let category: char = 'c';
    algorithm::get_best_seats_filtered_by_category(&mut stadium, &category, 3);
}
