use mpmcpq::PriorityQueue;
use mpmcpq::Stash;
use std::sync::Arc;
use tokio::sync::Mutex;
use rocket::tokio::sync::Notify;

#[derive(Debug)]
pub struct Buyer {
    pub buyer_id: String,
    pub seats: u32,
    pub category: char,
    pub notify: Arc<Notify>,
}

// Estructura que se usará como prioridad en la cola (cuantos más asientos, mayor prioridad)
impl PartialOrd for Buyer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.seats.cmp(&other.seats).reverse()) // Invertir para que más asientos sea mayor prioridad
    }
}

impl Ord for Buyer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.seats.cmp(&other.seats).reverse()
    }
}

impl PartialEq for Buyer {
    fn eq(&self, other: &Self) -> bool {
        self.seats == other.seats
    }
}

impl Eq for Buyer {}

pub struct AppState {
    pub priority_queue: Mutex<PriorityQueue<Buyer, u32>>, // Cola de prioridad gestionada con Mutex
}
