use std::collections::HashMap;
use crate::stadium::data;

enum Status {
    Purchased,
    Reserved,
    Available
}

struct Zone {
    name : String,
    prop : String,
    categories : HashMap<char, Category>
}

struct Category {
    rows : HashMap<char, Row>
}

struct Row {
    seats : HashMap<u8, Seat>
}

struct Seat {
    number : u8,
    visibility : f32,
    status : Status
}

pub fn a() {
    println!("this is the structures file.");
    data::b();
}