// main.rs

mod tasks;
mod types;
mod utils;

use std::collections::HashMap;

use types::{scheduler_input::SchedulerInput, scheduler_output::Slot};
use utils::json_reader::read_json;

use crate::{
    tasks::generator::generate_tasks,
    types::{
        scheduler_engine::{TBlockingSlotsMap, TBufferMap, TDueHrsMap, TFlexibleWeeklyGoals},
        scheduler_output::JSOutputSlots,
    },
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

    let mut scheduled: Vec<JSOutputSlots> = Vec::new();
    let mut impossible: Vec<JSOutputSlots> = Vec::new();

    generate_tasks(
        schedule,
        &mut calendar,
        &mut buffer,
        &mut due_task_hrs,
        &mut blocking_slots,
        &mut flexible_weekly_goals,
    );

    for key in 0..=6 {
        scheduled.insert(
            key,
            JSOutputSlots {
                day: key as i32,
                outputs: Vec::new(),
            },
        );
        impossible.insert(
            key,
            JSOutputSlots {
                day: key as i32,
                outputs: Vec::new(),
            },
        );
    }
    let mut dayItr = schedule.start_date.clone();
    for key in 0..=6 {
        let mut hrs_of_the_day: Vec<i32> = vec![-1; 24];
    }

    println!("{:#?}", calendar);

    // Now you can use the schedule struct as needed
}
