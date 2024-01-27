// main.rs

mod tasks;
mod types;
mod utils;

use std::collections::HashMap;

use chrono::Duration;
use types::{scheduler_input::SchedulerInput, scheduler_output::Slot};
use utils::json_reader::read_json;

use crate::{
    tasks::{generator::generate_tasks, operator::task_operator, scheduler::schedule_task},
    types::{
        scheduler_engine::{TBlockingSlotsMap, TBufferMap, TDueHrsMap, TFlexibleWeeklyGoals},
        scheduler_output::{FinalJSOutputSlot, JSOutputSlots},
    },
    utils::{constants::get_day_name, helpers::break_the_tree},
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
    let mut buffer_map: TBufferMap = HashMap::new();
    let mut due_task_hrs: TDueHrsMap = HashMap::new();
    let mut flexible_weekly_goals: TFlexibleWeeklyGoals = Vec::new();
    let mut blocking_slots_map: TBlockingSlotsMap = HashMap::new();

    let mut scheduled: Vec<JSOutputSlots> = Vec::new();
    let mut impossible: Vec<JSOutputSlots> = Vec::new();

    let goals = break_the_tree(schedule.goals);
    generate_tasks(
        goals.clone(),
        schedule.start_date.clone(),
        &mut calendar,
        &mut buffer_map,
        &mut due_task_hrs,
        &mut blocking_slots_map,
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
    let mut day_itr = schedule.start_date.clone();
    for key in 0..=6 {
        let mut hrs_of_the_day: Vec<i32> = vec![-1; 24];
        task_operator(
            key,
            &mut hrs_of_the_day,
            &mut calendar,
            &mut due_task_hrs,
            &mut buffer_map,
            schedule.start_date.clone(),
            &mut scheduled,
            &mut blocking_slots_map,
        );
        for weekly_goal in &mut flexible_weekly_goals {
            let goal = goals.get(&weekly_goal.slot.goalid.clone()).unwrap().clone();
            let actual_deadline = goal.deadline;
            if let Some(deadline) = actual_deadline {
                if deadline < day_itr {
                    continue;
                }
            }
            let day_name = get_day_name(day_itr.clone());
            if weekly_goal.valid_days.contains(&day_name) {
                let mut task = weekly_goal.slot.clone();
                let past_due = due_task_hrs.entry(task.goalid.clone()).or_insert(0);
                let due_duration = {
                    if past_due.clone() > 0 {
                        past_due.clone()
                    } else {
                        task.duration
                    }
                };
                *past_due = 0;
                if task.duration != 0 {
                    task.duration = due_duration;
                    schedule_task(
                        task.clone(),
                        key as i32,
                        schedule.start_date.clone(),
                        &mut hrs_of_the_day,
                        &mut due_task_hrs,
                        &mut blocking_slots_map,
                        &mut scheduled,
                    );
                    weekly_goal.slot = task.clone();
                }
            }
        }
        day_itr += Duration::days(1);
    }

    println!("{:#?}", scheduled);
    // let mut js_schedule: Vec<FinalJSOutputSlot> = Vec::new();
    // let impossible_schedule: Vec<FinalJSOutputSlot> = Vec::new();
    // let mut input_start_date = schedule.start_date.clone();

    // for key in 0..=6 {
    //     input_start_date += Duration::days(1);
    //     let slots_of_the_day = scheduled.get_mut(key + 1).unwrap();
    //     slots_of_the_day.outputs.sort_by(|a, b| {
    //         let s1 = a.start[11..13].parse::<u32>().unwrap();
    //         let s2 = b.start[11..13].parse::<u32>().unwrap();
    //         let e1 = a.start[14..16].parse::<u32>().unwrap();
    //         let e2 = b.start[14..16].parse::<u32>().unwrap();

    //         if s1 == s2 {
    //             e1.cmp(&e2)
    //         } else {
    //             s1.cmp(&s2)
    //         }
    //     });
    //     js_schedule.push(FinalJSOutputSlot {
    //         day: input_start_date.clone().to_string(),
    //         tasks: slots_of_the_day.outputs.clone(),
    //     });
    // }

    // println!("{:#?}", js_schedule);

    // Now you can use the schedule struct as needed
}
