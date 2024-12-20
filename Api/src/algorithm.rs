use crate::stadium::structures::Status::Reserved;
use crate::stadium::structures::{Category, Seat, Status, Zone};
use itertools::Itertools;
use rand::Rng;
use std::collections::{HashMap, HashSet};

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
        let general_candidates =
            get_available_combinations(all_available_seats, seats_quantity as usize);
        best_candidate = filter_candidates(general_candidates); // Chooses the best combination
        return best_candidate;
    }

    // Filters the best candidate among the best candidates from each category, meaning it gets the best set of seats from the entire zone
    best_candidate = filter_candidates(zone_candidates);
    return best_candidate;
}

fn get_category_candidate(
    category_available_seats: Vec<Vec<Seat>>,
    seats_quantity: u8,
) -> Vec<Seat> {
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
        let category_general_candidates =
            get_available_combinations(all_available_seats, seats_quantity as usize);
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
    let row_candidates: Vec<Vec<Seat>> =
        get_available_combinations(row_available_seats, seats_quantity as usize);

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
        if candidate
            .iter()
            .all(|&seat| seat.status == Status::Available)
        {
            available_combinations.push(candidate.into_iter().map(|seat| seat.clone()).collect());
        }
    }

    // Finally, it returns the list of available combinations that meet the criteria.
    return available_combinations;
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
        let mut candidates_seats_number: Vec<u8> =
            candidate.iter().map(|seat| seat.number).collect();
        candidates_seats_number.sort();

        // Now it sums the seat difference between the seats in the set, going from left to right
        let mut seats_difference: i8 = 0;

        if candidates_seats_number.len() > 0 {
            for i in 0..candidates_seats_number.len() - 1 {
                // The formula takes the absolute value (to always get a positive number) of the difference between the two seats minus one
                // This is done because, for example, the difference between seats 3 and 5 is just one seat (4), but their difference is 2
                // So one is subtracted, and the absolute value is taken as a precaution (it should always be positive, but this is safer)
                seats_difference +=
                    (candidates_seats_number[i + 1] as i8 - candidates_seats_number[i] as i8).abs()
                        - 1;
            }

            // Gets the visibility percentage of the current set of seats
            let candidate_visibility_average = get_candidate_visibility_average(candidate.clone());

            // Now it performs two comparisons: if the seat difference is less than the current seat difference (i.e., the seats are closer together),
            // it directly accepts the change and takes the new set as the best option so far
            // But if the difference is the same, it keeps the set with the better visibility percentage. In other words, it chooses the closest seats with the best visibility
            if (seats_difference < current_difference)
                || (seats_difference == current_difference
                    && candidate_visibility_average > current_candidate_visibility_average)
            {
                // This makes the candidate change if the conditions are met
                best_candidate = candidate.clone();
                current_difference = seats_difference;
                current_candidate_visibility_average = candidate_visibility_average;
            }
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

pub fn modify_seats_status(
    stadium: &mut HashMap<String, Zone>,
    candidate: Vec<Seat>,
    new_status: Status,
) {
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
        .map(
            |sub_vec| {
                sub_vec
                    .iter() // Iterate over the second level (Vec<Seat>)
                    .map(|inner_vec| inner_vec.len()) // Get the length of each sublist in the third level
                    .sum::<usize>()
            }, // Sum the lengths of all sublist in the second level
        )
        .sum::<usize>(); // Sum the lengths obtained from the first level
}

fn get_seats_in_different_zones(
    first_zone_candidates: Vec<Vec<Vec<Seat>>>,
    second_zone_candidates: Vec<Vec<Vec<Seat>>>,
    seats_quantity: u8,
) -> Vec<Seat> {
    // This function gets candidates in different zones, is used only if the stadium is almost filled
    let mut available_seats: Vec<Seat> = Vec::new();

    // First stores all available seats (in both zones) in a vector
    for (first_category, second_category) in first_zone_candidates
        .into_iter()
        .zip(second_zone_candidates.into_iter())
    {
        for (first_row, second_row) in first_category.into_iter().zip(second_category.into_iter()) {
            for seat in first_row {
                available_seats.push(seat);
            }
            for seat in second_row {
                available_seats.push(seat);
            }
        }
    }

    // Then, gets the best candidate by using combinations
    let candidates: Vec<Vec<Seat>> =
        get_available_combinations(available_seats, seats_quantity as usize);

    // From all the obtained combinations of seats, selects the best one, which will be the candidate for both zones
    let best_candidate: Vec<Seat> = filter_candidates(candidates);
    return best_candidate;
}

fn compare_zones_candidates(
    stadium: &mut HashMap<String, Zone>,
    seats_requested: u8,
    first_zone: String,
    second_zone: String,
) -> Vec<Seat> {
    // This function compares the candidates of two zones and returns the best candidate at all
    let mut zones_candidates: Vec<Vec<Seat>> = Vec::new(); // Stores the best seat candidates from each zone
    let mut best_seats: Vec<Seat> = Vec::new(); // Stores the final best seats selected from the candidates

    // Retrieve available seats for both zones
    let first_zone_candidates: Vec<Vec<Vec<Seat>>> =
        get_zone_available_seats(stadium.get(&first_zone).unwrap().clone());
    let second_zone_candidates: Vec<Vec<Vec<Seat>>> =
        get_zone_available_seats(stadium.get(&second_zone).unwrap().clone());

    // Compute the total number of available seats in each zone
    let first_zone_length: usize = get_zone_available_seats_quantity(&first_zone_candidates);
    let second_zone_length: usize = get_zone_available_seats_quantity(&second_zone_candidates);

    // Get the best candidate seats from each zone
    let best_first_zone_candidate: Vec<Seat> =
        get_zone_candidate(first_zone_candidates.clone(), seats_requested);
    let best_second_zone_candidate: Vec<Seat> =
        get_zone_candidate(second_zone_candidates.clone(), seats_requested);

    // If both best candidates are empty, the algorithm will get the best seats by mixing the zones
    // So this make possible to get a candidate like this [North, South, North], for example
    if best_first_zone_candidate.is_empty() && best_second_zone_candidate.is_empty() {
        best_seats = get_seats_in_different_zones(
            first_zone_candidates,
            second_zone_candidates,
            seats_requested,
        );
        return best_seats;
    }

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

pub fn fill_stadium(stadium: &mut HashMap<String, Zone>, fill_percentage: f32) {
    // This function fills up the stadium according to a fill percentage given
    let mut seats_quantity: u16 = 0; // This number represents the seats quantity that must be "purchased" to fill the stadium
    let mut purchased_seats_quantity: u16 = 0; // This number will be the quantity of "purchased seats" so far

    // This loop gets the total number of seats in the stadium
    for (zone_key, zone) in stadium.clone() {
        let zone_seats = get_zone_available_seats(zone);
        seats_quantity += get_zone_available_seats_quantity(&zone_seats) as u16;
    }

    // Calculate the number of seats that need to be filled to meet the fill percentage
    seats_quantity = (seats_quantity as f32 * fill_percentage).round() as u16;

    let mut rng = rand::thread_rng(); // Random number generator

    // While the current number of "purchased" seats is less than the target
    // It will iterate through the entire stadium, filling seats randomly
    while purchased_seats_quantity < seats_quantity {
        for (zone_key, zone) in stadium.clone() {
            let zone_seats = get_zone_available_seats(zone);
            for category in zone_seats {
                for row in category {
                    for seat in row {
                        // Generate a random number, 0 or 1
                        let random_number: u8 = rng.gen_range(0..=1);
                        // If the random number is 1 and the number of "purchased seats" so far has not exceeded the target, mark the seat as purchased
                        if (random_number == 1 && purchased_seats_quantity < seats_quantity) {
                            modify_seats_status(stadium, vec![seat], Status::Purchased);
                            purchased_seats_quantity += 1;
                        }
                    }
                }
            }
        }
    }
}

pub fn get_best_seats(
    stadium: &mut HashMap<String, Zone>,
    zone_requested: &String,
    seats_requested: u8,
) -> Vec<Seat> {
    // This is kindly the main function, is the function that must be called to initiate the algorithm
    let mut best_seats: Vec<Seat> = Vec::new(); // The best seats from the whole zones requested (shaded or sunny)

    // If the user selected shaded, it will search for seats in the North and South zones
    if (zone_requested == "shaded") {
        best_seats = compare_zones_candidates(
            stadium,
            seats_requested,
            "north".to_string(),
            "south".to_string(),
        );
        modify_seats_status(stadium, best_seats.clone(), Status::Reserved) // Changes the status to reserved
    } else {
        // If the user selected sunny, it will search for seats in the East and West zones
        best_seats = compare_zones_candidates(
            stadium,
            seats_requested,
            "east".to_string(),
            "west".to_string(),
        );
        modify_seats_status(stadium, best_seats.clone(), Status::Reserved) // Changes the status to reserved
    }
    return best_seats;
}

fn get_category_available_seats(chosen_category: &Category) -> Vec<Vec<Seat>> {
    let mut category_available_seats: Vec<Vec<Seat>> = Vec::new();

    for (row_key, row) in chosen_category.rows.iter() {
        let mut row_available_seats: Vec<Seat> = Vec::new();

        for (seat_key, seat) in row.seats.iter() {
            if seat.status == Status::Available {
                row_available_seats.push(seat.clone());
            }
        }

        category_available_seats.push(row_available_seats);
    }

    return category_available_seats;
}

fn get_worst_candidate(candidates_to_compare: Vec<Vec<Seat>>) -> Vec<Seat> {
    let mut worst_candidate: Vec<Seat> = Vec::new();
    let mut current_difference: i8 = -1;
    let mut current_candidate_visibility_average: f32 = 0.00;

    for candidate in candidates_to_compare.iter() {
        let mut candidates_seats_number: Vec<u8> =
            candidate.iter().map(|seat| seat.number).collect();
        candidates_seats_number.sort();

        let mut seats_difference: i8 = 0;

        if candidates_seats_number.len() > 0 {
            for i in 0..candidates_seats_number.len() - 1 {
                seats_difference +=
                    (candidates_seats_number[i + 1] as i8 - candidates_seats_number[i] as i8).abs()
                        - 1;
            }

            let candidate_visibility_average = get_candidate_visibility_average(candidate.clone());

            if (seats_difference > current_difference)
                || (seats_difference == current_difference
                    && candidate_visibility_average < current_candidate_visibility_average)
            {
                worst_candidate = candidate.clone();
                current_difference = seats_difference;
                current_candidate_visibility_average = candidate_visibility_average;
            }
        }
    }
    return worst_candidate;
}

fn are_candidates_equal(candidate1: &Vec<Seat>, candidate2: &Vec<Seat>) -> bool {
    if candidate1.len() != candidate2.len() {
        return false;
    }
    for (seat1, seat2) in candidate1.iter().zip(candidate2.iter()) {
        if !are_seats_equal(seat1, seat2) {
            return false;
        }
    }
    true
}

fn are_seats_equal(seat1: &Seat, seat2: &Seat) -> bool {
    seat1.zone.eq_ignore_ascii_case(&seat2.zone)
        && seat1.category == seat2.category
        && seat1.row == seat2.row
        && seat1.number == seat2.number
}

pub fn get_best_seats_filtered_by_category(
    stadium: &mut HashMap<String, Zone>,
    category_requested: &char,
    seats_requested: u8,
) -> Vec<Vec<Seat>> {
    let north_zone_available_seats_quantity: usize = get_zone_available_seats_quantity(
        &get_zone_available_seats(stadium.get("north").unwrap().clone()),
    );
    let south_zone_available_seats_quantity: usize = get_zone_available_seats_quantity(
        &get_zone_available_seats(stadium.get("south").unwrap().clone()),
    );
    let east_zone_available_seats_quantity: usize = get_zone_available_seats_quantity(
        &get_zone_available_seats(stadium.get("east").unwrap().clone()),
    );
    let west_zone_available_seats_quantity: usize = get_zone_available_seats_quantity(
        &get_zone_available_seats(stadium.get("west").unwrap().clone()),
    );

    let mut zones_with_seats_quantity: Vec<(&str, usize)> = vec![
        ("north", north_zone_available_seats_quantity),
        ("south", south_zone_available_seats_quantity),
        ("east", east_zone_available_seats_quantity),
        ("west", west_zone_available_seats_quantity),
    ];

    zones_with_seats_quantity.sort_by(|a, b| b.1.cmp(&a.1));

    let mut all_candidates: Vec<Vec<Seat>> = Vec::new();

    for (zone_name, _seats_quantity) in zones_with_seats_quantity {
        let zone = stadium.get(zone_name).unwrap();
        let mut category_available_seats: Vec<Vec<Seat>> = Vec::new();

        for (category_char, category) in &zone.categories {
            if category_char == category_requested {
                category_available_seats = get_category_available_seats(category);
                break;
            }
        }

        let category_best_seats = get_category_candidate(category_available_seats, seats_requested);
        all_candidates.push(category_best_seats);
    }

    all_candidates.retain(|vector| !vector.is_empty());

    if all_candidates.len() <= 3 {
        println!("less than three (or 3)");
        return all_candidates;
    }

    let all_candidates_copy = all_candidates.clone();
    let worst_candidate = get_worst_candidate(all_candidates_copy);

    let mut new_candidates: Vec<Vec<Seat>> = Vec::new();

    for candidate in all_candidates {
        if !are_candidates_equal(&candidate, &worst_candidate) {
            new_candidates.push(candidate);
        }
    }

    let new_candidates_copy = new_candidates.clone();
    for candidate in new_candidates_copy {
        println!("{:?}", candidate);
        modify_seats_status(stadium, candidate, Status::Reserved);
    }

    new_candidates
}

pub fn get_available_seats_by_zone(stadium: &HashMap<String, Zone>) -> HashMap<String, usize> {
    // This function retrieves the available seats by zone and returns them in a HashMap
    // It iterates over the entire stadium and counts the available seats in each zone
    // Then it stores the quantity in a HashMap, which is returned at the end
    let mut available_seats_by_zone: HashMap<String, usize> = HashMap::new();

    for (zone_key, zone) in stadium {
        let mut zone_available_seats = 0;
        for (_category_key, category) in &zone.categories {
            for (_row_key, row) in &category.rows {
                for (_seat_number, seat) in &row.seats {
                    if seat.status == Status::Available {
                        zone_available_seats += 1;
                    }
                }
            }
        }
        available_seats_by_zone.insert(zone_key.clone(), zone_available_seats);
    }

    available_seats_by_zone
}

pub fn get_available_seats_by_category(zone: &Zone) -> HashMap<String, usize> {
    let mut available_seats_by_category: HashMap<String, usize> = HashMap::new();

    for (category_key, category) in &zone.categories {
        let mut category_available_seats = 0;

        for (_row_key, row) in &category.rows {
            for (_seat_number, seat) in &row.seats {
                if seat.status == Status::Available {
                    category_available_seats += 1;
                }
            }
        }

        available_seats_by_category.insert(category_key.to_string(), category_available_seats);
    }

    available_seats_by_category
}

pub fn get_seats_by_zone_and_category(category: &Category) -> Vec<Vec<Seat>> {
    let mut rows_seats = Vec::new();

    // Iterar sobre cada fila en la categoría
    for row in category.rows.values() {
        // Recopilar todos los asientos en una fila y convertirlos en un vector
        let mut seats_in_row: Vec<Seat> = row.seats.values().cloned().collect();
        seats_in_row.sort_by_key(|seat| seat.number); // Ordenar por número de asiento
        rows_seats.push(seats_in_row);
        // Añadir el vector de asientos de esta fila al resultado
    }

    rows_seats
}
