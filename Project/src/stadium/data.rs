use std::collections::HashMap;
use crate::stadium::structures::{Category, Row, Seat, Status};

pub fn generate_data() -> Category  {

    let zone_names = vec!["Norte", "Sur", "Este", "Oeste"];
    let mut category_a : Category = Category::default();
    for zone in zone_names.iter() {
        //let mut categories = HashMap::new();

        // Category A
        // 5 Filas
        // (n \cdot 20) / 100
        let mut rows_a = HashMap::new();

        let row_1 = Row {
            seats: create_seats(1.00 * 25.00 / 100.00)
        };
        let row_2 = Row {
            seats: create_seats(2.00 * 25.00 / 100.00)
        };

        let row_3 = Row {
            seats: create_seats(3.00 * 25.00 / 100.00)
        };

        let row_4 = Row {
            seats: create_seats(4.00 * 25.00 / 100.00)
        };


        rows_a.insert('W', row_1);
        rows_a.insert('X', row_2);
        rows_a.insert('Y', row_3);
        rows_a.insert('Z', row_4);

        category_a.rows = rows_a;
    }
    return category_a
}

pub fn create_seats(row_visibility: f32) -> HashMap<u8, Seat> {
    // % de visibilidad
    // 5 filas 5 asientos

    let mut seats = HashMap::new();

    for seat_number in 1..=10 {
        let mut visibility = row_visibility;

        // Ajuste de visibilidad para los asientos centrales (4, 5, 6)
        if seat_number >= 4 && seat_number <= 6 {
            visibility += row_visibility * 0.02; // Aumenta un 2% de la visibilidad base

            // Asegúrate de que la visibilidad no exceda 1.0
            if visibility > 1.0 {
                visibility = 1.0;
            }
        }
        seats.insert(
            seat_number,
            Seat {
                number: seat_number,
                visibility,
                status: Status::Available,
            },
        );
    }
    return seats
}