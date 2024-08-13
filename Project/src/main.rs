use std::collections::HashMap;

mod stadium;

fn main() {
    //let mut stadium = HashMap::new();
    let categoria = stadium::data::generate_data();
    println!("{:?}", categoria.rows.get(&'Y').unwrap().seats);

}
