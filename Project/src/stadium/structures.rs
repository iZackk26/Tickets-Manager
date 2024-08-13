use std::collections::HashMap;
use crate::stadium::data;

pub enum Status {
    Purchased,
    Reserved,
    Available,
}

pub struct Zone {
    pub name: String,
    pub prop: String,
    pub categories: HashMap<char, Category>,
}

pub struct Category {
    pub rows: HashMap<char, Row>,
}

pub struct Row {
    pub seats: HashMap<u8, Seat>,
}

pub struct Seat {
    pub number: u8,
    pub visibility: f32,
    pub status: Status,
}

pub fn a() {
    println!("this is the structures file.");
}