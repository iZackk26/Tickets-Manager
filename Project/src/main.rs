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
    //algorithm::test(&stadium);

    let numbers = vec![1,6,7];
    let mut difference_2: i32 = 0;

    for i in 0..numbers.len() - 1 {
        let difference = (numbers[i + 1] as i32 - numbers[i] as i32).abs() - 1;
        difference_2 += difference;
        println!("Diferencia entre {} y {}: {}", numbers[i], numbers[i + 1], difference);
    }
    println!("{}", difference_2);


}
