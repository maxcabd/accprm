use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub accessories: Vec<Entry>,
}

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub accessory_name: String,
    pub accessory_id: String,
    pub modelcode: String,
    pub icon_filepath: String,
    pub head_a: bool,
    pub head_b: bool,
    pub face: bool,
    pub eyes: bool,
    pub back: bool,
    pub back_pocket: bool,
    pub tail: bool,
    pub arms: bool,
}

impl Config {
    pub fn read_cfg(filepath: &str) -> Self {
        let json_str = std::fs::read_to_string(filepath).unwrap();
        serde_json::from_str(&json_str).unwrap()
    }
}
