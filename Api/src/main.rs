mod algorithm;
mod priorityQueue;
mod stadium;

#[macro_use]
extern crate rocket;

use crate::algorithm::{
    fill_stadium, get_available_seats_by_category, get_available_seats_by_zone,
    get_best_seats_filtered_by_category, get_seats_by_zone_and_category, modify_seats_status,
};

use crate::priorityQueue::{AppState, Buyer};
use crate::stadium::structures::{Seat, SeatingMap, StadiumState, Status as SeatStatus, Zone};
use mpmcpq::PriorityQueue;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::futures::TryFutureExt;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::tokio::sync::Notify;
use rocket::State;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

fn make_cors() -> Cors {
    CorsOptions {
        // Permitir todos los orígenes
        allowed_origins: AllowedOrigins::all(),
        // Permitir métodos comunes
        allowed_methods: vec![
            rocket::http::Method::Get,
            rocket::http::Method::Post,
            rocket::http::Method::Put,
            rocket::http::Method::Delete,
            rocket::http::Method::Options,
            rocket::http::Method::Head,
        ]
        .into_iter()
        .map(|m| From::from(m))
        .collect(),
        // Permitir todos los encabezados
        allowed_headers: AllowedHeaders::all(),
        // Permitir credenciales
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error al construir CORS")
}

#[derive(Debug, Serialize, Deserialize)]
struct Asiento {
    cantidad: u32,
    categoria: char,
}

// Función que procesa la cola de prioridad
async fn process_priority_queue(app_state: Arc<AppState>, stadium_state: Arc<StadiumState>) {
    loop {
        let mut queue_guard = app_state.priority_queue.lock().await;

        // Intentar recibir el siguiente mensaje con mayor prioridad
        let buyer_msg = queue_guard.try_recv();

        if let Some(buyer) = buyer_msg {
            if let Some(buyer_data) = buyer.message() {
                println!(
                    "Procesando comprador con ID: {} y {} asientos",
                    buyer_data.buyer_id, buyer_data.seats
                );

                // Obtener acceso al estadio gestionado
                let mut stadium = stadium_state.seating_map.lock().await;

                // Buscar los mejores asientos
                let best_seats = get_best_seats_filtered_by_category(
                    &mut stadium,
                    &buyer_data.category,
                    buyer_data.seats as u8,
                );
                let mut best_seats_lock = buyer_data.best_seats.lock().await;
                *best_seats_lock = Some(best_seats);

                // Notificar al comprador que su solicitud fue procesada
                buyer_data.notify.notify_one();

                println!(
                    "Solicitud procesada para el comprador ID: {}",
                    buyer_data.buyer_id
                );
            }
        } else {
            // Si no hay solicitudes, dormir un breve momento para no consumir CPU
            rocket::tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }
}

#[get("/asientos")]
async fn get_stadium(stadium_state: &State<Arc<StadiumState>>) -> Result<Json<SeatingMap>, Status> {
    // Bloquear el acceso seguro al estado del estadio de forma asíncrona
    let stadium_guard = stadium_state.seating_map.lock().await;

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

// Ruta para manejar solicitudes GET con datos JSON
#[rocket::get("/get-seats/<category>/<quantity>", format = "json")]
async fn get_seats(
    category: String,
    quantity: u32,
    stadium_state: &State<Arc<StadiumState>>,
    app_state: &State<Arc<AppState>>,
) -> Result<Json<Vec<Vec<Seat>>>, Status> {
    if quantity <= 0 {
        return Err(Status::BadRequest);
    }

    let category_char = category.chars().next().ok_or(Status::BadRequest)?;

    let notify = Arc::new(Notify::new());
    let best_seats = Arc::new(Mutex::new(None)); // Crear el campo `best_seats` inicialmente vacío

    let buyer = Buyer {
        buyer_id: "some_unique_id".to_string(),
        seats: quantity,
        category: category_char,
        notify: notify.clone(),
        best_seats: best_seats.clone(), // Pasa `best_seats` al comprador
    };

    let priority = quantity;

    {
        let mut queue_guard = app_state.priority_queue.lock().await;
        queue_guard.send_nostash(buyer, priority);
    }

    // Esperar a que la solicitud del cliente sea procesada
    notify.notified().await;

    // Leer el valor de `best_seats` después de la notificación
    let best_seats_lock = best_seats.lock().await;
    if let Some(ref best_seats) = *best_seats_lock {
        Ok(Json(best_seats.clone()))
    } else {
        Err(Status::InternalServerError)
    }
}


#[post("/modify-seats", format = "json", data = "<seats>")]
async fn modify_seats(
    seats: Json<Vec<Seat>>, // Recibe la lista de asientos en formato JSON
    stadium_state: &State<Arc<StadiumState>>, // Usa el estadio gestionado en el estado global
) -> Result<status::Custom<&'static str>, Status> {
    // Obtener el estado actual del estadio de forma asíncrona
    let mut stadium = stadium_state.seating_map.lock().await;

    // Llama a la función para modificar el estado de los asientos a "Purchased"
    modify_seats_status(
        &mut stadium,
        seats.into_inner(),
        crate::stadium::structures::Status::Purchased,
    );

    // Retorna una respuesta 200 indicando que los asientos fueron modificados exitosamente
    Ok(status::Custom(
        Status::Ok,
        "Asientos modificados correctamente",
    ))
}

#[get("/available-seats-by-zone")]
async fn get_available_seats_by_zone_route(
    stadium_state: &State<Arc<StadiumState>>,
) -> Json<HashMap<String, usize>> {
    // Obtener el estadio gestionado en el estado global de forma asíncrona
    let stadium = stadium_state.seating_map.lock().await;

    // Obtener los asientos disponibles por zona
    let available_seats_by_zone = get_available_seats_by_zone(&stadium);

    // Devolver los asientos por zona en formato JSON
    Json(available_seats_by_zone)
}

#[get("/available-seats-by-category/<zone_name>")]
async fn get_available_seats_by_category_route(
    stadium_state: &State<Arc<StadiumState>>,
    zone_name: String,
) -> Json<HashMap<String, usize>> {
    // Acceder al estado del estadio y obtener el mapa de asientos
    let stadium = stadium_state.seating_map.lock().await;

    // Buscar la zona especificada
    if let Some(zone) = stadium.get(&zone_name) {
        // Llamar a la función para obtener los asientos disponibles por categoría
        let available_seats_by_category = get_available_seats_by_category(zone);
        Json(available_seats_by_category)
    } else {
        // Enviar una respuesta vacía si la zona no existe
        Json(HashMap::new())
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

// Fairing personalizado para iniciar la tarea en segundo plano
pub struct QueueProcessor;

#[rocket::async_trait]
impl Fairing for QueueProcessor {
    fn info(&self) -> Info {
        Info {
            name: "Queue Processor",
            kind: Kind::Liftoff,
        }
    }

    async fn on_liftoff(&self, rocket: &rocket::Rocket<rocket::Orbit>) {
        // Obtén el estado gestionado
        let app_state = rocket
            .state::<Arc<AppState>>()
            .expect("AppState missing")
            .clone();
        let stadium_state = rocket
            .state::<Arc<StadiumState>>()
            .expect("StadiumState missing")
            .clone();

        // Inicia la tarea en segundo plano dentro del runtime de Tokio
        rocket::tokio::spawn(async move {
            process_priority_queue(app_state, stadium_state).await;
        });
    }
}

#[get("/seats/<zone>/<category>")]
async fn get_seats_by_zone_and_category_route(
    stadium_state: &rocket::State<Arc<StadiumState>>,
    zone: String,
    category: String,
) -> Json<Vec<Vec<Seat>>> {
    // Convertir `category` a `char`
    if let Some(category_char) = category.chars().next() {
        // Bloquea el mapa de asientos para acceder de forma segura
        let stadium = stadium_state.seating_map.lock().await;

        // Obtiene la zona especificada del estadio
        if let Some(zone_data) = stadium.get(&zone) {
            // Obtiene la categoría especificada dentro de la zona
            if let Some(category_data) = zone_data.categories.get(&category_char) {
                // Llama a `get_seats_by_row` para obtener los asientos organizados por filas
                let rows_seats = get_seats_by_zone_and_category(category_data);

                // Devuelve los datos como JSON
                return Json(rows_seats);
            }
        }
    }

    // Si la zona o categoría no existe o la conversión falla, devuelve un vector vacío
    Json(Vec::new())
}


// Función principal para lanzar el servidor
#[launch]
fn rocket() -> _ {
    let mut stadium: HashMap<String, Zone> = stadium::data::generate_stadium();
    fill_stadium(&mut stadium, 0.4); // Llena el estadio con asientos y otras propiedades

    let priority_queue = PriorityQueue::<Buyer, u32>::new();

    let app_state = Arc::new(AppState {
        priority_queue: Mutex::new(priority_queue), // Usar tokio::sync::Mutex aquí
    });

    let stadium_state = Arc::new(StadiumState {
        seating_map: Mutex::new(stadium), // Usar tokio::sync::Mutex aquí también
    });

    rocket::build()
        .manage(stadium_state.clone()) // Gestiona el estado del estadio
        .manage(app_state.clone()) // Gestiona el estado de la cola de prioridad
        .mount(
            "/",
            routes![
                index,
                get_stadium,
                get_seats,
                modify_seats,
                get_available_seats_by_zone_route,
                get_available_seats_by_category_route,
                get_seats_by_zone_and_category_route
            ],
        )
        .register("/", catchers![not_found])
        .attach(QueueProcessor) // Adjunta el Fairing aquí
        .attach(make_cors()) // Adjunta el fairing de CORS
}
