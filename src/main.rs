// main.rs

mod utils;
mod types;
mod tasks;

use utils::json_reader::read_json;
use types::{scheduler_input::SchedulerInput, scheduler_output::Slot};

use crate::tasks::generator::generate_tasks;

fn main() {

    let file_path = "./src/tests/input.json";
    let schedule: SchedulerInput = read_json(file_path);

    let mut calendar: Vec<Vec<Slot>> = vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
 
    generate_tasks(schedule, &calendar);
    // println!("{:#?}", schedule);
    // Now you can use the schedule struct as needed

}
