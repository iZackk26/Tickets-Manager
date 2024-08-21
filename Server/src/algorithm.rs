use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::stadium::structures::{Category, Seat, Status, Zone};
use crate::stadium::structures::Status::Reserved;

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
    let mut best_candidate: Vec<Seat> = Vec::new();
    let mut zone_candidates: Vec<Vec<Seat>> = Vec::new();

    // For each category in the zone, it calls the function that retrieves the best seats from the entire category and stores this
    // in the list of possible candidates
    for category in zone_categories.iter() {
        zone_candidates.push(get_category_candidate(category.clone(), seats_quantity));
    }

    // If no best candidate was found in any row (from the previous loop, it gets something like: [ [], [], [], [] ]
    // It needs to combine the available seats across the entire zone and choose those with the best visibility
    if zone_candidates.iter().all(|sublist| sublist.is_empty()) {
        let mut all_available_seats: Vec<Seat> = Vec::new();
        for category in zone_categories.iter() {
            for row in category.iter() {
                for seat in row.iter() {
                    all_available_seats.push(seat.clone());
                }
            }
        }

        // Now that it has all available seats, it calls the combinations function to get every combination possible
        let general_candidates = get_available_combinations(all_available_seats, seats_quantity as usize);
        best_candidate = filter_candidates(general_candidates); // Chooses the best combination
        return best_candidate;

    }

    // Filters the best candidate among the best candidates from each category, meaning it gets the best set of seats from the entire zone
    best_candidate = filter_candidates(zone_candidates);
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
                all_available_seats.push(seat.clone());
            }
        }

        // Now that it has all available seats, it calls the combinations function to get every combination possible
        let category_general_candidates = get_available_combinations(all_available_seats, seats_quantity as usize);
        best_candidate = filter_candidates(category_general_candidates); // Chooses the best combination
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

    // Gets every seat combination possible
    let row_candidates: Vec<Vec<Seat>> = get_available_combinations(row_available_seats, seats_quantity as usize);

    // From all the obtained combinations of seats, selects the best one, which will be the candidate for the current row
    row_candidate = filter_candidates(row_candidates);
    return row_candidate;
}

fn get_available_combinations(seats: Vec<Seat>, seats_quantity: usize) -> Vec<Vec<Seat>> {
    // This variable will store all possible combinations of available seats.
    let mut available_combinations: Vec<Vec<Seat>> = Vec::new();

    // At this point, it combines all available seats and stores the combinations in the previous variable.
    // You could see something like C(A,R), where A = number of available seats, R = number of requested seats.
    for candidate in seats.iter().combinations(seats_quantity) {
        // If all seats in the combination are available, the combination is added to the list of available combinations.
        if candidate.iter().all(|&seat| seat.status == Status::Available) {
            available_combinations.push(candidate.into_iter().map(|seat| seat.clone()).collect());
        }
    }

    // Finally, it returns the list of available combinations that meet the criteria.
    return available_combinations
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

fn get_zone_available_seats_quantity(zone: &Vec<Vec<Vec<Seat>>>) -> usize {
    // This function gets the quantity of available seats (from a zone)
    return zone
        .iter() // Iterate over the first level (Vec<Vec<Seat>>)
        .map(|sub_vec|
             sub_vec.iter() // Iterate over the second level (Vec<Seat>)
                 .map(|inner_vec| inner_vec.len()) // Get the length of each sublist in the third level
                 .sum::<usize>() // Sum the lengths of all sublist in the second level
        )
        .sum::<usize>() // Sum the lengths obtained from the first level
}

fn compare_zones_candidates(stadium: &mut HashMap<String, Zone>, seats_requested: u8, first_zone: String, second_zone: String) -> Vec<Seat> {
    // This function compares the candidates of two zones and returns the best candidate at all
    let mut zones_candidates: Vec<Vec<Seat>> = Vec::new(); // Stores the best seat candidates from each zone
    let mut best_seats: Vec<Seat> = Vec::new(); // Stores the final best seats selected from the candidates

    // Retrieve available seats for both zones
    let first_zone_candidates: Vec<Vec<Vec<Seat>>> = get_zone_available_seats(stadium.get(&first_zone).unwrap().clone());
    let second_zone_candidates: Vec<Vec<Vec<Seat>>> = get_zone_available_seats(stadium.get(&second_zone).unwrap().clone());

    // Compute the total number of available seats in each zone
    let first_zone_length: usize = get_zone_available_seats_quantity(&first_zone_candidates);
    let second_zone_length: usize = get_zone_available_seats_quantity(&second_zone_candidates);

    // Get the best candidate seats from each zone
    let best_first_zone_candidate: Vec<Seat> = get_zone_candidate(first_zone_candidates.clone(), seats_requested);
    let best_second_zone_candidate: Vec<Seat> = get_zone_candidate(second_zone_candidates.clone(), seats_requested);

    // Compare the total number of available seats in each zone and store candidates accordingly
    if first_zone_length > second_zone_length {
        zones_candidates.push(best_second_zone_candidate); // Add the second zone's best candidate if it has fewer available seats
        zones_candidates.push(best_first_zone_candidate); // Add the first zone's best candidate if it has more available seats
    } else {
        zones_candidates.push(best_first_zone_candidate); // Add the first zone's best candidate if it has fewer available seats
        zones_candidates.push(best_second_zone_candidate); // Add the second zone's best candidate if it has more available seats
    }

    // Filter the candidates to select the best option from both zones
    best_seats = filter_candidates(zones_candidates);
    return best_seats; // Return the final best seats
}


pub fn get_best_seats(stadium: &mut HashMap<String, Zone>, zone_requested: String, seats_requested: u8) {
    // This is kindly the main function, is the function that must be called to initiate the algorithm
    let mut best_seats: Vec<Seat> = Vec::new(); // The best seats from the whole zones requested (shaded or sunny)

    // If the user selected shaded, it will search for seats in the North and South zones
    if (zone_requested == "shaded") {
        best_seats = compare_zones_candidates(stadium, seats_requested, "north".to_string(), "south".to_string());
        modify_seats_status(stadium, best_seats.clone(), Status::Reserved) // Changes the status to reserved
    } else { // If the user selected sunny, it will search for seats in the East and West zones
        best_seats = compare_zones_candidates(stadium, seats_requested, "east".to_string(), "west".to_string());
        modify_seats_status(stadium, best_seats.clone(), Status::Reserved) // Changes the status to reserved
    }

    // Prints the modified seats from the original stadium
    for (zone_key, zone) in stadium.iter() {
        for (category_key, category) in zone.categories.iter() {
            let mut category_available_seats: Vec<Vec<Seat>> = Vec::new();
            for (row_key, row) in category.rows.iter() {
                let mut row_available_seats: Vec<Seat> = Vec::new();
                for (seat_key, seat) in row.seats.iter() {
                    if seat.status == Status::Reserved {
                        println!("{:#?}", seat)
                    }
                }
            }
        }
    }
}