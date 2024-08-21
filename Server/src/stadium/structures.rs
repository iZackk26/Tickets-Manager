use std::collections::HashMap;

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

#[derive(Debug, Default, Clone)]
pub struct Zone {
    pub prop: String,
    pub categories: HashMap<char, Category>,
}

#[derive(Debug, Default, Clone)]
pub struct Category {
    pub rows: HashMap<char, Row>,
}

#[derive(Debug, Default, Clone)]
pub struct Row {
    pub seats: HashMap<u8, Seat>,
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