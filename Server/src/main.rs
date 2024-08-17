use std::collections::HashMap;
use crate::stadium::structures::{Status, Zone};

mod stadium;
mod algorithm;
mod server;

fn main() {
    let mut stadium : HashMap<String, Zone> = stadium::data::generate_stadium();

    //println!("{:?}", stadium.get("north").unwrap().categories.get(&'a').unwrap().rows.get(&'w').unwrap().seats);
    //algorithm::test(&stadium);
    server::socket::server();
}