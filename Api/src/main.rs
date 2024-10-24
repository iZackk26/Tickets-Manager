mod algorithm;
mod stadium;
mod priorityQueue;

#[macro_use]
extern crate rocket;

use crate::algorithm::{fill_stadium, get_available_seats_by_zone, get_best_seats_filtered_by_category, modify_seats_status};
use crate::stadium::structures::{Seat, SeatingMap, StadiumState, Zone};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use mpmcpq::PriorityQueue;
use rocket::tokio::sync::Notify;
use crate::priorityQueue::{AppState, Buyer};

#[derive(Debug, Serialize, Deserialize)]
struct Asiento {
    cantidad: u32,
    categoria: char,
}

// Función que procesa la cola de prioridad
async fn process_priority_queue(
    app_state: &AppState,
    stadium_state: &rocket::State<StadiumState>,
) -> Option<Vec<Vec<Seat>>> {
    let mut queue_guard = app_state.priority_queue.lock().unwrap();

    // Intentar recibir el siguiente mensaje con mayor prioridad
    let buyer_msg = queue_guard.try_recv();

    match buyer_msg {
        Some(buyer) => {
            // Usar el método `message()` y manejar el caso de `None`
            if let Some(buyer_data) = buyer.message() {
                println!(
                    "Procesando comprador con ID: {} y {} asientos",
                    buyer_data.buyer_id, buyer_data.seats
                );

                // Obtener acceso al estadio gestionado
                let mut stadium = stadium_state.seating_map.lock().unwrap();

                // Buscar los mejores asientos
                let best_seats = get_best_seats_filtered_by_category(
                    &mut stadium,
                    &buyer_data.category,
                    buyer_data.seats as u8,
                );

                // Notificar al comprador que su solicitud fue procesada
                buyer_data.notify.notify_one();

                // Devolver los mejores asientos
                Some(best_seats)
            } else {
                println!("No se pudo obtener el mensaje de Buyer.");
                None
            }
        }
        None => {
            println!("No hay solicitudes en la cola.");
            None
        }
    }
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
#[rocket::get("/get-seats", format = "json", data = "<data>")]
async fn get_seats(
    data: Json<Asiento>,
    stadium_state: &rocket::State<StadiumState>,
    app_state: &rocket::State<AppState>,
) -> Result<Json<Vec<Vec<Seat>>>, Status> {
    if data.cantidad <= 0 {
        return Err(Status::BadRequest);
    }

    let notify = Arc::new(Notify::new());
    let buyer = Buyer {
        buyer_id: "some_unique_id".to_string(),
        seats: data.cantidad,
        category: data.categoria,
        notify: notify.clone(),
    };

    let priority = data.cantidad;

    // Añadir el comprador a la cola de prioridad
    {
        let mut queue_guard = app_state.priority_queue.lock().map_err(|_| Status::InternalServerError)?;
        queue_guard.send_nostash(buyer, priority);
    }

    // Esperar a que la solicitud del cliente sea procesada
    notify.notified().await;

    // Una vez notificado, procesar la solicitud
    let mut stadium = stadium_state.seating_map.lock().map_err(|_| Status::InternalServerError)?;

    // Obtener los mejores asientos una vez que la solicitud es procesada
    let best_seats = get_best_seats_filtered_by_category(&mut stadium, &data.categoria, data.cantidad as u8);

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

    let priority_queue = PriorityQueue::<Buyer, u32>::new();

    let app_state = AppState {
        priority_queue: Mutex::new(priority_queue),
    };

    // Crea el estado gestionado con la estructura StadiumState
    let stadium_state = StadiumState {
        seating_map: Mutex::from(stadium),
    };

    rocket::build()
        .manage(stadium_state) // Gestiona el estado del estadio
        .manage(app_state) // Gestiona el estado de la cola de prioridad
        .mount("/", routes![index, get_stadium, get_seats, modify_seats, get_available_seats_by_zone_route])
        .register("/", catchers![not_found])
}
