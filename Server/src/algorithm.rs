use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::stadium::structures::{Category, Seat, Status, Zone};


fn get_zone_available_seats(chosen_zone: Zone) -> Vec<Vec<Vec<Seat>>> {
    // This is the first function executed in the algorithm, it retrieves all available seats in the zone (North, South, East, or West)
    // It iterates over the entire zone and adds seats with status 'Available' to lists (distributes them by row and category)
    // The list looks something like this: [ [[Available seats from row W], [Available seats from row X]...], [[Available seats from row W], [Available seats from row X]...]]
    // In other words: [ Category(Rows(Seats)), Category(Rows(Seats)), Category(Rows(Seats)), Category(Rows(Seats))]
    let mut zone_available_seats: Vec<Vec<Vec<Seat>>> = Vec::new();
    for (category_key, category) in chosen_zone.categories.iter() {
        let mut category_available_seats: Vec<Vec<Seat>> = Vec::new();
        for (row_key, row) in category.rows.iter() {
            let mut row_available_seats: Vec<Seat> = Vec::new();
            for (seat_key, seat) in row.seats.iter() {
                if seat.status == Status::Available {
                    row_available_seats.push(seat.clone());
                }
            }
            category_available_seats.push(row_available_seats);
        }
        zone_available_seats.push(category_available_seats);
    }
    return zone_available_seats;
}

fn get_zone_candidate(zone_categories: Vec<Vec<Vec<Seat>>>, seats_quantity: u8) -> Vec<Seat> {
    // This function retrieves the best candidate from the entire zone

    let mut zone_candidates: Vec<Vec<Seat>> = Vec::new();

    // For each category in the zone, it calls the function that retrieves the best seats from the entire category and stores this
    // in the list of possible candidates
    for category in zone_categories.iter() {
        zone_candidates.push(get_category_candidate(category.clone(), seats_quantity));
    }

    // Filters the best candidate among the best candidates from each category, meaning it gets the best set of seats from the entire zone
    let best_candidate: Vec<Seat> = filter_candidates(zone_candidates);
    return best_candidate;
}

fn get_category_candidate(category_available_seats: Vec<Vec<Seat>>, seats_quantity: u8) -> Vec<Seat> {
    // This function retrieves the best candidate from a category (considering the best candidates from each row)
    let mut category_candidates: Vec<Vec<Seat>> = Vec::new(); // The best candidate from each row
    let mut best_candidate: Vec<Seat> = Vec::new(); // The best candidate from the category
    let mut current_category_available_seats: u8 = 0; // The number of available seats in the entire category

    // Checks if the category has available seats
    for row in category_available_seats.iter() {
        for seat in row {
            if seat.status == Status::Available {
                current_category_available_seats += 1;
            }
        }
    }

    // If it doesn't have enough available seats, it exits and looks in the next category
    if seats_quantity > current_category_available_seats {
        return best_candidate;
    }

    // For each row, it retrieves the best candidate from it (a candidate is a vector of seats)
    for row in category_available_seats.iter() {
        let row_candidates = get_row_candidate(row.clone(), seats_quantity);
        category_candidates.push(row_candidates);
    }

    // If no best candidate was found in any row (from the previous loop, it gets something like: [ [], [], [], [] ]
    // It needs to combine the available seats across the entire category and choose those with the best visibility
    if category_candidates.iter().all(|sublist| sublist.is_empty()) {
        let mut all_available_seats: Vec<Seat> = Vec::new();
        for row in category_available_seats.iter() {
            for seat in row.iter() {
                all_available_seats.push(seat.clone()); // Note the use of clone here
            }
        }

        let mut category_general_candidates: Vec<Vec<Seat>> = Vec::new();
        // At this point, it combines all available seats and stores the combinations in the previous variable
        // You could see something like C(A,R), where A = number of available seats, R = number of requested seats
        for seat in all_available_seats.iter().combinations(seats_quantity as usize) {
            if seat.iter().all(|&seat| seat.status == Status::Available) {
                category_general_candidates.push(seat.into_iter().map(|seat| seat.clone()).collect());
            }
        }
        best_candidate = filter_candidates(category_general_candidates); // Chooses the best from all combinations and returns it
        return best_candidate;
    }

    // If there's at least one row that meets the required number of available seats, it selects the best among them
    best_candidate = filter_candidates(category_candidates);

    return best_candidate;
}

fn get_row_candidate(row_available_seats: Vec<Seat>, seats_quantity: u8) -> Vec<Seat> {
    // This function gets the best candidate in a whole row
    let mut row_candidate: Vec<Seat> = Vec::new(); // The best set of seats from the row
    let mut current_row_available_seats: u8 = 0; // The number of available seats in the row

    // Checks if the row has enough available seats to satisfy the requested quantity
    for seat in row_available_seats.iter() {
        if seat.status == Status::Available {
            current_row_available_seats += 1;
        }
    }

    // If it doesn't have enough, returns an empty list and continues with the next row
    if seats_quantity > current_row_available_seats {
        return row_candidate;
    }

    let mut row_candidates: Vec<Vec<Seat>> = Vec::new(); // Here, the candidates for the best set of seats in the row will be stored
    // At this point, it combines all available seats and stores the combinations in the previous variable
    // You could see something like C(A,R), where A = number of available seats, R = number of requested seats
    for candidate in row_available_seats.iter().combinations(seats_quantity as usize) {
        if candidate.iter().all(|&seat| seat.status == Status::Available) {
            row_candidates.push(candidate.into_iter().map(|seat| seat.clone()).collect());
        }
    }

    // From all the obtained combinations of seats, selects the best one, which will be the candidate for the current row
    row_candidate = filter_candidates(row_candidates);
    return row_candidate;
}

fn filter_candidates(candidates_to_compare: Vec<Vec<Seat>>) -> Vec<Seat> {
    let mut best_candidate: Vec<Seat> = Vec::new(); // This is the best set of seats filtered
    // The current seat difference starts at 11 since it will never exceed this value. It's the sum of the differences between each seat in the set
    let mut current_difference: i8 = 11; // The seat difference refers to their numbers, e.g., between seat 2 and seat 5, the difference is 2 (3,4)
    let mut current_candidate_visibility_average: f32 = 0.00; // Visibility percentage of the entire set

    // For each candidate received, it first calculates the seat number difference between each set, then calls the function
    // to get the visibility percentage and compares them
    for candidate in candidates_to_compare.iter() {
        // This gets the number of each seat, stores it in a vector, and sorts it
        // For example, if the current set is seat 3, seat 7, and seat 4, the list will be [3,4,7]
        let mut candidates_seats_number: Vec<u8> = candidate.iter().map(|seat| seat.number).collect();
        candidates_seats_number.sort();

        // Now it sums the seat difference between the seats in the set, going from left to right
        let mut seats_difference: i8 = 0;
        for i in 0..candidates_seats_number.len() - 1 {
            // The formula takes the absolute value (to always get a positive number) of the difference between the two seats minus one
            // This is done because, for example, the difference between seats 3 and 5 is just one seat (4), but their difference is 2
            // So one is subtracted, and the absolute value is taken as a precaution (it should always be positive, but this is safer)
            seats_difference += (candidates_seats_number[i + 1] as i8 - candidates_seats_number[i] as i8).abs() - 1;
        }

        // Gets the visibility percentage of the current set of seats
        let candidate_visibility_average = get_candidate_visibility_average(candidate.clone());

        // Now it performs two comparisons: if the seat difference is less than the current seat difference (i.e., the seats are closer together),
        // it directly accepts the change and takes the new set as the best option so far
        // But if the difference is the same, it keeps the set with the better visibility percentage. In other words, it chooses the closest seats with the best visibility
        if (seats_difference < current_difference) || (seats_difference == current_difference && candidate_visibility_average > current_candidate_visibility_average) {
            // This makes the candidate change if the conditions are met
            best_candidate = candidate.clone();
            current_difference = seats_difference;
            current_candidate_visibility_average = candidate_visibility_average;
        }
    }

    return best_candidate;
}

fn get_candidate_visibility_average(candidate: Vec<Seat>) -> f32 {
    // Here the average visibility percentage of the set of seats will be stored
    // The function is very simple: it sums the visibility percentages of each seat
    // And finally, it divides by the number of seats in the set
    let mut candidate_visibility_average: f32 = 0.00;

    for seat in &candidate {
        candidate_visibility_average += seat.visibility;
    }

    candidate_visibility_average = candidate_visibility_average / candidate.len() as f32;
    return candidate_visibility_average;
}

fn modify_seats_status(stadium: &mut HashMap<String, Zone>, candidate: Vec<Seat>, new_status: Status) {
    // This function modifies the status of the seats passed as a parameter
    // It receives a mutable reference to the stadium to make changes directly to the stadium
    // In other words, it changes the original seats. The function iterates through the stadium using keys
    // Until it finds the seats in the set and assigns them the status passed as a parameter
    // You need to work with get_mut to change the original reference
    for seat in candidate {
        if let Some(zone) = stadium.get_mut(&seat.zone.to_string()) {
            if let Some(category) = zone.categories.get_mut(&seat.category) {
                if let Some(row) = category.rows.get_mut(&seat.row) {
                    if let Some(seat_reference) = row.seats.get_mut(&seat.number) {
                        seat_reference.status = new_status;
                    }
                }
            }
        }
    }
}

pub fn get_best_seats(stadium: &mut HashMap<String, Zone>) {
    let user_chosen_zone: String = String::from("shaded"); //sombra
    let seats_requested: u8 = 1;
    if (user_chosen_zone == "shaded") {
        //println!("{:#?}", stadium.get("north").unwrap().categories.get(&'a').unwrap().rows.get(&'w').unwrap().seats.get(&1).unwrap());
        let north_zone_candidates : Vec<Vec<Vec<Seat>>> = get_zone_available_seats(stadium.get("north").unwrap().clone());
        //println!("{:#?}", get_row_candidate(north_zone_candidates[0][0].clone(), seats_requested));
        let north_candidate: Vec<Seat> = get_zone_candidate(north_zone_candidates, seats_requested);
        println!("{:?}", north_candidate);
        modify_seats_status(stadium, north_candidate.clone(), Status::Reserved);
        for seat in &north_candidate {
            println!(
                "{:#?}",
                stadium
                    .get(&seat.zone)
                    .unwrap()
                    .categories
                    .get(&seat.category)
                    .unwrap()
                    .rows
                    .get(&seat.row)
                    .unwrap()
                    .seats
                    .get(&seat.number)
                    .unwrap()
            );
        }

        //println!("{:#?}", north_candidate_reference);
        //let south_zone_candidates : Vec<Vec<Vec<&Seat>>> = get_zone_available_seats(stadium.get("south").unwrap());
    } else {
        //let east_zone_candidates : Vec<Vec<Vec<&Seat>>> = get_zone_available_seats(stadium.get("east").unwrap());
        //let west_zone_candidates : Vec<Vec<Vec<&Seat>>> = get_zone_available_seats(stadium.get("west").unwrap());
    }
}