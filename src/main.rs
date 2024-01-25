// main.rs

mod tasks;
mod types;
mod utils;

use std::collections::HashMap;

use types::{scheduler_input::SchedulerInput, scheduler_output::Slot};
use utils::json_reader::read_json;

use crate::{
    tasks::generator::generate_tasks,
    types::scheduler_engine::{TBufferMap, TDueHrsMap},
};

fn print(mut x: i32) {
    x = x + 2;
    println!("{}", x);
}
fn main() {
    let file_path = "./src/tests/input.json";
    let schedule: SchedulerInput = read_json(file_path);

    let mut calendar: Vec<Vec<Slot>> = vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];
    let mut buffer: TBufferMap = HashMap::new();
    let mut due_task_hrs: TDueHrsMap = HashMap::new();
    generate_tasks(schedule, &mut calendar, &mut buffer, &mut due_task_hrs);
    let mut x = 2;
    print(x);
    println!("{}", x);
    // println!("{:#?}", schedule);
    // Now you can use the schedule struct as needed
}
