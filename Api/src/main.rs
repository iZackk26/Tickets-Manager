mod algorithm;
mod stadium;

#[macro_use]
extern crate rocket;

use crate::algorithm::{fill_stadium, get_available_seats_by_zone, get_best_seats_filtered_by_category, modify_seats_status};
use crate::stadium::structures::{Seat, SeatingMap, StadiumState, Zone};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
struct Asiento {
    cantidad: u32,
    categoria: char,
}

#[get("/asientos")]
fn get_stadium(stadium_state: &State<StadiumState>) -> Result<Json<SeatingMap>, Status> {
    // Bloquear el acceso seguro al estado del estadio
    let stadium_guard = stadium_state.seating_map.lock().unwrap();

    // Mapeamos el HashMap al SeatingMap
    let seating_map = SeatingMap {
        north: stadium_guard.get("north").cloned().unwrap_or_default(),
        south: stadium_guard.get("south").cloned().unwrap_or_default(),
        east: stadium_guard.get("east").cloned().unwrap_or_default(),
        west: stadium_guard.get("west").cloned().unwrap_or_default(),
    };

    // Devolver el SeatingMap dentro de un Json
    Ok(Json(seating_map))
}

// Ruta para manejar solicitudes POST
#[get("/get-seats", format = "json", data = "<data>")]
fn get_seats(
    data: Json<Asiento>,
    stadium_state: &rocket::State<StadiumState>  // Añadir el estadio gestionado como argumento
) -> Result<Json<Vec<Vec<Seat>>>, Status> {
    if data.cantidad <= 0 {
        return Err(Status::BadRequest);
    }

    // Obtener acceso al estadio gestionado, y bloquearlo para su acceso
    let mut stadium = stadium_state.seating_map.lock().map_err(|_| Status::InternalServerError)?;

    // Llamar a la función que encuentra los mejores asientos
    let best_seats = get_best_seats_filtered_by_category(&mut stadium, &data.categoria, data.cantidad as u8);

    // Devolver los asientos encontrados en formato JSON
    Ok(Json(best_seats))
}

#[post("/modify-seats", format = "json", data = "<seats>")]
fn modify_seats(
    seats: Json<Vec<Seat>>, // Recibe la lista de asientos en formato JSON
    stadium_state: &State<StadiumState>, // Usa el estadio gestionado en el estado global
) -> Result<status::Custom<&'static str>, Status> {
    // Obtener el estado actual del estadio
    let mut stadium = stadium_state.seating_map.lock().unwrap();

    // Llama a la función para modificar el estado de los asientos a "ocupados" o el estado que elijas
    modify_seats_status(&mut stadium, seats.into_inner(), crate::stadium::structures::Status::Purchased);  // Cambia el estado de los asientos a "Purchased"

    // Retorna una respuesta 200 indicando que los asientos fueron modificados exitosamente
    Ok(status::Custom(Status::Ok, "Asientos modificados correctamente"))
}

#[get("/available-seats-by-zone")]
fn get_available_seats_by_zone_route(stadium_state: &State<StadiumState>) -> Json<HashMap<String, usize>> {

    // Obtener el estadio gestionado en el estado global
    let stadium = stadium_state.seating_map.lock().unwrap();

    // Obtener los asientos disponibles por zona
    let available_seats_by_zone = get_available_seats_by_zone(&stadium);

    // Devolver los asientos por zona en formato JSON
    Json(available_seats_by_zone)
}



#[get("/")]
fn index() -> &'static str {
    "¡Servidor Rocket en funcionamiento!"
}

// Manejador de errores personalizado para solicitudes no encontradas
#[catch(404)]
fn not_found() -> &'static str {
    "¡Página no encontrada! Verifica la URL."
}

// Función principal para lanzar el servidor
#[launch]
fn rocket() -> _ {
    //Genera el estadio inicializado
    let mut stadium: HashMap<String, Zone> = stadium::data::generate_stadium();
    fill_stadium(&mut stadium, 0.0); // Llena el estadio con asientos y otras propiedades
                                     //println!("{:?}", stadium);

    // Crea el estado gestionado con la estructura StadiumState
    let stadium_state = StadiumState {
        seating_map: Mutex::from(stadium),
    };

    rocket::build()
        .manage(stadium_state) // Gestiona el estado del estadio
        .mount("/", routes![index, get_stadium, get_seats, modify_seats, get_available_seats_by_zone_route])
        .register("/", catchers![not_found])
}
