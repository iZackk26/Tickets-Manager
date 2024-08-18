use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    Purchased,
    Reserved,
    Available,
}

impl Default for Status {
    fn default() -> Self {
        Status::Available
    }
}

#[derive(Debug, Default)]
pub struct Zone {
    pub prop: String,
    pub categories: RwLock<HashMap<char, Category>>,
}


impl Zone {
    pub fn get_categories(&self) -> std::sync::RwLockReadGuard<HashMap<char, Category>> {
        self.categories.read().unwrap()
    }
}

#[derive(Debug, Default)]
pub struct Category {
    pub rows: RwLock<HashMap<char, Row>>,
}

impl Category {
    pub fn get_rows(&self) -> std::sync::RwLockReadGuard<HashMap<char, Row>> {
        self.rows.read().unwrap()
    }
}

#[derive(Debug, Default)]
pub struct Row {
    pub seats: RwLock<HashMap<u8, Seat>>,
}
impl Row {
    pub fn get_seats(&self) -> std::sync::RwLockReadGuard<HashMap<u8, Seat>> {
        self.seats.read().unwrap()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Seat {
    pub zone: String,
    pub category: char,
    pub row: char,
    pub number: u8,
    pub visibility: f32,
    pub status: Status,
}