use serde_json;
use std::{fs::File, io::Read};

use crate::types::scheduler_input::SchedulerInput;


pub fn read_json(file_path: &str) -> SchedulerInput {
    // Open the file
    let file = File::open(file_path).expect("Failed to open file");

    // Read the file into a string
    let mut buf_reader = std::io::BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("Failed to read file");

    // Parse the JSON string into a Schedule struct
    serde_json::from_str(&contents).expect("Failed to parse JSON")
}