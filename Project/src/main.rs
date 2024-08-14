use std::collections::HashMap;
use crate::stadium::structures::Zone;

mod stadium;

fn main() {
    let mut stadium : HashMap<String, Zone> = stadium::data::generate_stadium();
    // println!("{:?}", stadium.get("Norte").unwrap().categories.get(&'A').unwrap().rows.len());
    // println!("{:?}", stadium.get("Norte").unwrap().categories.get(&'A').unwrap().rows.get(&'W').unwrap().seats.len());
    println!("{:?}", stadium.get("Sur").unwrap().categories.get(&'C').unwrap().rows.get(&'Y').unwrap().seats.get(&5));

}
