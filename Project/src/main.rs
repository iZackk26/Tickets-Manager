use std::collections::HashMap;
use crate::stadium::structures::{Status, Zone};

mod stadium;
mod algorithm;

fn main() {
    let mut stadium : HashMap<String, Zone> = stadium::data::generate_stadium();
    // Asumiendo que `stadium` es un HashMap o similar.
    if let Some(zone) = stadium.get_mut("north") {
        if let Some(category) = zone.categories.get_mut(&'a') {
            if let Some(row) = category.rows.get_mut(&'w') {
                if let Some(seat) = row.seats.get_mut(&3) {
                    seat.status = Status::Purchased;
                }
            }
        }
    }
    // Asumiendo que `stadium` es un HashMap o similar.
    if let Some(zone) = stadium.get_mut("north") {
        if let Some(category) = zone.categories.get_mut(&'a') {
            if let Some(row) = category.rows.get_mut(&'w') {
                if let Some(seat) = row.seats.get_mut(&6) {
                    seat.status = Status::Purchased;
                }
            }
        }
    }


    //println!("{:?}", stadium.get("north").unwrap().categories.get(&'a').unwrap().rows.get(&'w').unwrap().seats);
    algorithm::test(&stadium);
}
