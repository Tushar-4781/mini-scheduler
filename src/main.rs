// main.rs

mod utils;
mod types;
mod tasks;

use utils::json_reader::read_json;
use types::scheduler_input::SchedulerInput;

use crate::tasks::generator::generate_tasks;

fn main() {
    // Your JSON file path
    let file_path = "./src/tests/input.json";
    let schedule: SchedulerInput = read_json(file_path);
    generate_tasks(schedule);
    // println!("{:#?}", schedule);
    // Now you can use the schedule struct as needed
}
