use std::collections::HashMap;
use crate::stadium::structures::{Category, Seat, Status, Zone};


fn get_zone_candidates(chosen_zone: &Zone) {
    let mut category_available_seats: Vec<Vec<Vec<&Seat>> = Vec::new();
    for (category_key, category) in chosen_zone.categories.iter() {
        let mut row_available_seats: Vec<&Seat> = Vec::new();
        for (row_key, row) in category.rows.iter() {
            for (seat_key, seat) in row.seats.iter() {
                if (seat.status == Status::Available) {
                    row_available_seats.push(seat)
                }
            }
        }
        category_available_seats.push(row_available_seats);
    }
    println!("{:?}", category_available_seats[0]);
}

pub fn test(stadium: & HashMap<String, Zone>) {
    let user_chosen_zone: String = String::from("sombra");
    if (user_chosen_zone == "sombra") {
        get_zone_candidates(stadium.get("north").unwrap());
    }

}