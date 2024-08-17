use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::stadium::structures::{Category, Seat, Status, Zone};

fn get_zone_candidates(chosen_zone: &Zone) -> Vec<Vec<Vec<&Seat>>> {
    let mut zone_available_seats: Vec<Vec<Vec<&Seat>>> = Vec::new();
    for (category_key, category) in chosen_zone.categories.iter() {
        let mut category_available_seats: Vec<Vec<&Seat>> = Vec::new();
        for (row_key, row) in category.rows.iter() {
            let mut row_available_seats: Vec<&Seat> = Vec::new();
            for (seat_key, seat) in row.seats.iter() {
                if (seat.status == Status::Available) {
                    row_available_seats.push(seat)
                }
            }
            category_available_seats.push(row_available_seats);
        }
        zone_available_seats.push(category_available_seats);
    }
    return zone_available_seats
}

fn get_category_candidates(category_available_seats: Vec<Vec<&Seat>>, seats_quantity: u8) -> Vec<Vec<&Seat>> {
    let mut category_candidates: Vec<Vec<&Seat>> = Vec::new();
    let mut current_category_available_seats: u8 = 0;

    for row in category_available_seats.iter() {
        for seat in row {
            if (seat.status == Status::Available) {
                current_category_available_seats += 1;
            }
        }
    }

    if (seats_quantity > current_category_available_seats) {
        return category_candidates
    }

    return category_candidates
}

fn get_row_candidates(row_available_seats: Vec<&Seat>, seats_quantity: u8) -> Vec<&Seat> {
    let mut row_candidate: Vec<&Seat> = Vec::new();
    let mut current_row_available_seats: u8 = 0;

    for seat in row_available_seats.iter() {
        if (seat.status == Status::Available) {
            current_row_available_seats += 1;
        }
    }

    if (seats_quantity > current_row_available_seats) {
        return row_candidate
    }

    let mut row_candidates: Vec<Vec<&Seat>> = Vec::new();
    for candidate in row_available_seats.iter().combinations(seats_quantity as usize) {
        if candidate.iter().all(|&&seat| seat.status == Status::Available) {
            row_candidates.push(candidate.into_iter().map(|&seat| seat).collect());
        }
    }

    row_candidate = filter_row_candidates(row_candidates);
    return row_candidate
}

fn filter_row_candidates(row_candidates: Vec<Vec<&Seat>>) -> Vec<&Seat> {
    let mut row_candidate: Vec<&Seat> = Vec::new();

    let mut candidates_difference: HashMap<i8, &Vec<&Seat>> = HashMap::new();
    for candidate in row_candidates.iter() {
        let mut candidates_seats_number: Vec<u8> = Vec::new();
        for seat in candidate.iter() {
            candidates_seats_number.push(seat.number);
        }
        candidates_seats_number.sort();

        let mut seats_difference: i8 = 0;
        for i in 0..candidates_seats_number.len() - 1 {
            seats_difference += (candidates_seats_number[i + 1] as i8 - candidates_seats_number[i] as i8).abs() - 1;
        }
        candidates_difference.insert(seats_difference, candidate);
    }

    let mut current_difference: i8 = 11;
    for (difference, candidate) in candidates_difference {
        if (difference < current_difference) {
            row_candidate = candidate.clone();
            current_difference = difference;
        }
    }
    return row_candidate
}

pub fn test(stadium: & HashMap<String, Zone>) {
    let user_chosen_zone: String = String::from("shaded"); //sombra
    let seats_requested: u8 = 5;
    if (user_chosen_zone == "shaded") {
        let north_zone_candidates : Vec<Vec<Vec<&Seat>>> = get_zone_candidates(stadium.get("north").unwrap());
        println!("{:?}", get_row_candidates(north_zone_candidates[0][0].clone(), seats_requested))
        //let south_zone_candidates : Vec<Vec<Vec<&Seat>>> = get_zone_candidates(stadium.get("south").unwrap());
    } else {
        let east_zone_candidates : Vec<Vec<Vec<&Seat>>> = get_zone_candidates(stadium.get("east").unwrap());
        let west_zone_candidates : Vec<Vec<Vec<&Seat>>> = get_zone_candidates(stadium.get("west").unwrap());
    }
}