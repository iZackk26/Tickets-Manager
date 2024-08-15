use std::collections::HashMap;
use crate::stadium::structures::Zone;

mod stadium;

fn main() {
    let mut stadium : HashMap<String, Zone> = stadium::data::generate_stadium();
    println!("{:?}", stadium.get("North").unwrap().categories.get(&'A').unwrap().rows.get(&'W').unwrap().seats);
}
