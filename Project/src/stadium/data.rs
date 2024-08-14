use std::collections::HashMap;
use crate::stadium::structures::{Category, Row, Seat, Status};

pub fn b() {
    println!("this is the data file.")
}

fn define_data() {
    let seat_1 = Seat {
        number: 1,
        visibility: 0.1,
        status: Status::Available
    };
        let seat_1 = Seat {
        number: 1,
        visibility: 0.1,
        status: Status::Available
    };
        let seat_1 = Seat {
        number: 1,
        visibility: 0.1,
        status: Status::Available
    };    let seat_1 = Seat {
        number: 1,
        visibility: 0.1,
        status: Status::Available
    };

}

pub fn generate_data()  {

    let zone_names = vec!["Norte", "Sur", "Este", "Oeste"];
    let mut category_a : Category;
    for zone in zone_names.iter() {
        //let mut categories = HashMap::new();

        // Category A
        // 5 Filas
        // (n \cdot 20) / 100
        let mut rows_a = HashMap::new(); // Rows list
        let row_1 = Row {
            seats: create_row(1.00 * 25.00 / 100.00) // Seats list
        };

        rows_a.insert('W', row_1);

        for row in rows_a {
            println!("{:?}", row);
        }

        category_a.rows = rows_a;

    }
    return rows_a


}

fn create_row(row_visibility: f32) -> HashMap<u8, Seat>{
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
    seats
}