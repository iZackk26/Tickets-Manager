use std::collections::{HashMap, HashSet};
use crate::stadium::structures::{Category, Row, Seat, Status, Zone};

pub fn generate_stadium() -> HashMap<String, Zone> {

    let zone_names : Vec<&str> = vec!["Norte", "Sur", "Este", "Oeste"];
    let categories_names : Vec<char> = vec!['A', 'B', 'C', 'D'];
    let mut visibility_rate : f32 = 0.00;
    let mut zones : HashMap<String, Zone> = HashMap::new();

    for zone_name in zone_names.iter() {
        let mut zone : Zone = Zone::default();

        for category_name in categories_names.iter() {
            let mut category : Category = Category::default();
            let mut rows : HashMap<char, Row> = HashMap::new();

            if category_name == &'B' || category_name == &'C' {
                visibility_rate = 0.00;
            }  else {
                visibility_rate = 5.00;
            }

            let row_1 = Row {
                seats: create_rows(1.00 * (25.00 - visibility_rate) / 100.00, 10)
            };
            let row_2 = Row {
                seats: create_rows(2.00 * (25.00 - visibility_rate)  / 100.00, 10)
            };

            let row_3 = Row {
                seats: create_rows(3.00 * (25.00 - visibility_rate)  / 100.00, 10)
            };

            let row_4 = Row {
                seats: create_rows(4.00 * (25.00 - visibility_rate) / 100.00, 10)
            };

            rows.insert('W', row_1);
            rows.insert('X', row_2);
            rows.insert('Y', row_3);
            rows.insert('Z', row_4);

            category.rows = rows;
            zone.categories.insert(*category_name, category);
        }
        zones.insert(zone_name.to_string(), zone);
    }
    return zones
}

pub fn create_rows(row_visibility: f32, seats_quantiy : u8) -> HashMap<u8, Seat> {
    // % de visibilidad
    // 5 filas 5 asientos

    let mut seats = HashMap::new();

    for seat_number in 1..= seats_quantiy {
        let mut visibility = row_visibility;

        // Ajuste de visibilidad para los asientos centrales (4, 5, 6)
        if seat_number >= 4 && seat_number <= 6 {
            visibility += row_visibility * 0.5; // Aumenta un 2% de la visibilidad base

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