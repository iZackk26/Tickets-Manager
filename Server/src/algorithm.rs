use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::stadium::structures::{Category, Seat, Status, Zone};


fn get_zone_available_seats(chosen_zone: &Zone) -> Vec<Vec<Vec<&Seat>>> {
    let mut zone_available_seats: Vec<Vec<Vec<&Seat>>> = Vec::new();
    for (category_key, category) in chosen_zone.categories.iter() {
        let mut category_available_seats: Vec<Vec<&Seat>> = Vec::new();
        for (row_key, row) in category.rows.iter() {
            let mut row_available_seats: Vec<&Seat> = Vec::new();
            for (seat_key, seat) in row.seats.iter() {
                if seat.status == Status::Available {
                    row_available_seats.push(seat);
                }

            }
            category_available_seats.push(row_available_seats);
        }
        zone_available_seats.push(category_available_seats);
    }
    return zone_available_seats
}

fn get_zone_candidate() {
    // AQUI HAY QUE RECORRER CADA SUBLISTA QUE DA LA FUNCION DE ARRIBA Y A CADA UNA APLICARLE UN get_category_cadidate
    // ADEMAS TENER EN CUENTA EL CASO DE QUE ESTÉ MUY LLENO
}

fn get_category_candidate<'a>(category_available_seats: &'a Vec<Vec<&'a Seat>>, seats_quantity: u8) -> Vec<&'a Seat> {
    let mut category_candidates: Vec<Vec<&'a Seat>> = Vec::new(); // El mejor candidato de cada fila
    let mut best_candidate: Vec<&'a Seat> = Vec::new(); // El mejor candidato de la categoria
    let mut current_category_available_seats: u8 = 0; // La cantidad de asientos disponibles en toda la categoria

    // Verifica que la categoria tenga asientos disponibles
    for row in category_available_seats.iter() {
        for seat in row {
            if seat.status == Status::Available {
                current_category_available_seats += 1;
            }
        }
    }

    // Si no los tiene, se sale y busca en la siguiente categoria
    if seats_quantity > current_category_available_seats {
        return best_candidate;
    }

    // Por cada fila, obtiene el mejor candidato de ella (un candidato es un vector de asientos)
    for row in category_available_seats.iter() {
        let row_candidates = get_row_candidate(row, seats_quantity);
        category_candidates.push(row_candidates);
    }

    // Si no encontro un mejor candidato de ninguna fila (del for anterior se obtiene algo asi: [ [], [], [], []]
    // Hay que hacer una combinacion de los asientos disponibles en toda la categoria y escoger los que tengan mejor visibilidad
    if (category_candidates.iter().all(|sublist| sublist.is_empty())) {
        let mut all_available_seats: Vec<&Seat> = Vec::new();
        for row in category_available_seats.iter() {
            for seat in row.iter() {
                all_available_seats.push(seat.clone()) // fijarse en esto por los clone -------------------------------------------------------------------------------------
            }
        }

        let mut category_general_candidates: Vec<Vec<&Seat>> = Vec::new();
        for seat in all_available_seats.iter().combinations(seats_quantity as usize) {
            if seat.iter().all(|&&seat| seat.status == Status::Available) {
                category_general_candidates.push(seat.into_iter().map(|&seat| seat).collect());
            }
        }
        best_candidate = filter_candidates(category_general_candidates); // Escoge el mejor de todas las combinaciones y retorna
        return best_candidate
    }

    // Si existe al menos una fila que cumpla con tener los asientos disponibles, la va a escoger (asimismo escoge entre todas las opciones, la mejor)
    best_candidate = filter_candidates(category_candidates);

    return best_candidate
}

fn get_row_candidate<'a>(row_available_seats: &'a Vec<&'a Seat>, seats_quantity: u8) -> Vec<&'a Seat> {
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

    row_candidate = filter_candidates(row_candidates);
    return row_candidate
}

fn filter_candidates(row_candidates: Vec<Vec<&Seat>>) -> Vec<&Seat> {
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
    let mut current_candidate_visibility_average: f32 = 0.00;
    for (difference, candidate) in candidates_difference {
        if (difference < current_difference) {
            row_candidate = candidate.clone();
            current_difference = difference;
        } else if (difference == current_difference && get_candidate_visibility_average(candidate) > current_candidate_visibility_average) {
            row_candidate = candidate.clone();
            current_candidate_visibility_average = get_candidate_visibility_average(candidate)
        }
    }
    return row_candidate
}

fn get_candidate_visibility_average(candidate: &Vec<&Seat>) -> f32 {
    let mut candidate_visibility_average: f32 = 0.00;

    for seat in candidate {
        candidate_visibility_average += seat.visibility
    }

    candidate_visibility_average = candidate_visibility_average / candidate.len() as f32;
    return candidate_visibility_average
}


pub fn test(stadium: & HashMap<String, Zone>) {
    let user_chosen_zone: String = String::from("shaded"); //sombra
    let seats_requested: u8 = 5;
    if (user_chosen_zone == "shaded") {
        let north_zone_candidates : Vec<Vec<Vec<&Seat>>> = get_zone_available_seats(stadium.get("north").unwrap());
        let candidate = (&north_zone_candidates[0], seats_requested);
        //let south_zone_candidates : Vec<Vec<Vec<&Seat>>> = get_zone_available_seats(stadium.get("south").unwrap());
    } else {
        let east_zone_candidates : Vec<Vec<Vec<&Seat>>> = get_zone_available_seats(stadium.get("east").unwrap());
        let west_zone_candidates : Vec<Vec<Vec<&Seat>>> = get_zone_available_seats(stadium.get("west").unwrap());
    }
}