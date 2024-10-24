mod algorithm;
mod stadium;

#[macro_use] extern crate rocket;

use std::collections::HashMap;
use std::sync::Mutex;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::response::status;
use rocket::State;
use crate::algorithm::fill_stadium;
use crate::stadium::structures::{SeatingMap, StadiumState, Zone};

#[derive(Debug, Serialize, Deserialize)]
struct Asiento {
    cantidad: u32,
    categoria: char,
}

#[get("/asientos")]
fn listar_asientos(stadium_state: &State<StadiumState>) -> Result<Json<SeatingMap>, Status> {
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
#[post("/get-seats", format = "json", data = "<data>")]
fn crear_datos(data: Json<Asiento>) -> Result<Json<Asiento>, Status> {

    if data.cantidad  <= 0 {
        Err(Status::BadRequest)
    } else {
        println!("Datos recibidos: {:?}", data);
        Ok(data)
    }
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
    fill_stadium(&mut stadium, 0.0);  // Llena el estadio con asientos y otras propiedades
    //println!("{:?}", stadium);

    // Crea el estado gestionado con la estructura StadiumState
    let stadium_state = StadiumState {
        seating_map: Mutex::from(stadium),
    };

    rocket::build()
        .manage(stadium_state)  // Gestiona el estado del estadio
        .mount("/", routes![index, listar_asientos, crear_datos])
        .register("/", catchers![not_found])
}

