use std::collections::HashMap;
use std::sync::Arc;

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
    pub categories: Arc<HashMap<char, Category>>,
}

impl Zone {
    pub fn get_categories(&self) -> Arc<HashMap<char, Category>> {
        Arc::clone(&self.categories)
    }
}

#[derive(Debug, Default)]
pub struct Category {
    pub rows: Arc<HashMap<char, Row>>,
}

impl Category {
    pub fn get_rows(&self) -> Arc<HashMap<char, Row>> {
        Arc::clone(&self.rows)
    }
}

#[derive(Debug, Default)]
pub struct Row {
    pub seats: Arc<HashMap<u8, Seat>>,
}

impl Row {
    pub fn get_seats(&self) -> Arc<HashMap<u8, Seat>> {
        Arc::clone(&self.seats)
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
