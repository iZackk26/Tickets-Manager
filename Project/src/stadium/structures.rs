use std::collections::HashMap;

#[derive(Debug)]
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
    pub categories: HashMap<char, Category>,
}

#[derive(Debug, Default)]
pub struct Category {
    pub rows: HashMap<char, Row>,
}

#[derive(Debug, Default)]
pub struct Row {
    pub seats: HashMap<u8, Seat>,
}

#[derive(Debug, Default)]
pub struct Seat {
    pub number: u8,
    pub visibility: f32,
    pub status: Status,
}