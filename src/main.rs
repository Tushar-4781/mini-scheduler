// main.rs

mod utils;
mod types;

// use utils::json_reader::read_json;
// use types::scheduler_input::SchedulerInput;
use types::scheduler_input::SchedulerInput;
// json_reader.rs

use serde_json;

use std::fs::File;
use std::io::Read;


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

fn main() {
    // Your JSON file path
    let file_path = "/tests/input.json";
    let mut file = File::open(file_path).expect("Could not find the file on given path");

    // read the file
    let mut content = String::new();
    // priocess of reading the file
    file.read_to_string(&mut content).expect("Opps! Can't read the file");

    // Call the read_json function from the json_reader module
    let schedule: SchedulerInput = read_json(file_path);

    // Now you can use the schedule struct as needed
    println!("{:#?}", schedule);
}
