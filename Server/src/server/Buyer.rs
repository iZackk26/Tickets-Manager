use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Buyer {
    pub SectionType: String,
    pub Quantity: u8,
    pub ResponseTime: u32,
    pub Response: bool,
}