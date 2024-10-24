use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Zone {
    pub prop: String,
    pub categories: HashMap<char, Category>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Category {
    pub category: char,
    pub rows: HashMap<char, Row>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Row {
    pub seats: HashMap<u8, Seat>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Seat {
    pub zone: String,
    pub category: char,
    pub row: char,
    pub number: u8,
    pub visibility: f32,
    pub status: Status,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Default)]
pub enum Status {
    #[default]
    Purchased,
    Reserved,
    Available,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeatingMap {
    pub south: Zone,
    pub west: Zone,
    pub north: Zone,
    pub east: Zone,
}

pub struct StadiumState {
    pub seating_map: Mutex<HashMap<String, Zone>>,
}
