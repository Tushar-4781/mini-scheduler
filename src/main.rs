// main.rs

mod tasks;
mod types;
mod utils;

use std::collections::HashMap;

use types::{scheduler_input::SchedulerInput, scheduler_output::Slot};
use utils::json_reader::read_json;

use crate::{
    tasks::generator::generate_tasks,
    types::scheduler_engine::{TBlockingSlotsMap, TBufferMap, TDueHrsMap, TFlexibleWeeklyGoals},
};

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
    let mut flexible_weekly_goals: TFlexibleWeeklyGoals = Vec::new();
    let mut blocking_slots: TBlockingSlotsMap = HashMap::new();

    generate_tasks(
        schedule,
        &mut calendar,
        &mut buffer,
        &mut due_task_hrs,
        &mut blocking_slots,
        &mut flexible_weekly_goals,
    );

    // println!("{:#?}", schedule);
    // Now you can use the schedule struct as needed
}
