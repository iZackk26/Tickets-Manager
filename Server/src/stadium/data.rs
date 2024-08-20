use std::collections::HashMap;
use crate::stadium::structures::{Category, Row, Seat, Status, Zone};
use std::sync::Arc;

pub fn generate_stadium() -> HashMap<String, Zone> {

    let zone_names: Vec<&str> = vec!["north", "south", "east", "west"];
    let categories_names: Vec<char> = vec!['a', 'b', 'c', 'd'];
    let mut visibility_rate: f32 = 0.00;
    let mut zones: HashMap<String, Zone> = HashMap::new();

    for zone_name in zone_names.iter() {
        let mut zone: Zone = Zone::default();
        let mut categories: HashMap<char, Category> = HashMap::new();

        for category_name in categories_names.iter() {
            let mut category: Category = Category::default();
            let mut rows: HashMap<char, Row> = HashMap::new();

            if category_name == &'b' || category_name == &'c' {
                visibility_rate = 0.00;
            } else {
                visibility_rate = 5.00;
            }

            let row_1 = Row {
                seats: create_rows(zone_name.to_string(), *category_name, 'w', 1.00 * (25.00 - visibility_rate) / 100.00, 10),
            };
            let row_2 = Row {
                seats: create_rows(zone_name.to_string(), *category_name, 'x', 2.00 * (25.00 - visibility_rate) / 100.00, 10),
            };

            let row_3 = Row {
                seats: create_rows(zone_name.to_string(), *category_name, 'y', 3.00 * (25.00 - visibility_rate) / 100.00, 10),
            };

            let row_4 = Row {
                seats: create_rows(zone_name.to_string(), *category_name, 'z', 4.00 * (25.00 - visibility_rate) / 100.00, 10),
            };

            rows.insert('w', row_1);
            rows.insert('x', row_2);
            rows.insert('y', row_3);
            rows.insert('z', row_4);

            category.rows = Arc::new(rows);
            categories.insert(*category_name, category);
        }

        zone.categories = Arc::new(categories);
        zones.insert(zone_name.to_string(), zone);
    }
    zones
}

pub fn create_rows(zone_name: String, category_name: char, row_name: char, row_visibility: f32, seats_quantity: u8) -> Arc<HashMap<u8, Seat>> {
    let mut seats = HashMap::new();

    for seat_number in 1..=seats_quantity {
        let mut visibility = row_visibility;
        if seat_number >= 4 && seat_number <= 6 {
            visibility += row_visibility * 1.25;
            if visibility > 1.0 {
                visibility = 1.0;
            }
        }
        seats.insert(
            seat_number,
            Seat {
                zone: zone_name.clone(),
                category: category_name,
                row: row_name,
                number: seat_number,
                visibility,
                status: Status::Available,
            },
        );
    }
    Arc::new(seats) // Devuelve el HashMap envuelto en Arc
}