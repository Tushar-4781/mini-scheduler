use std::iter::repeat;

use chrono::{Datelike, NaiveDateTime};

use crate::{
    types::{
        scheduler_engine::{TBlockingSlotsMap, TDueHrsMap},
        scheduler_output::{JSOutputSlots, OutputSlot, Slot},
    },
    utils::helpers::format_date,
};

use super::generator::gen_and_push_impossible_task;

pub fn schedule_task(
    task: Slot,
    day: i32,
    start_date: NaiveDateTime,
    hrs_of_the_day: &mut Vec<i32>,
    due_task_hrs: &mut TDueHrsMap,
    blocking_slots_map: &mut TBlockingSlotsMap,
    scheduled: &mut Vec<JSOutputSlots>,
) {
    let mut start_hr_exists = true;
    let mut start_hr: i32 = task.start;
    let deadline = task.deadline;
    while hrs_of_the_day[start_hr as usize] != -1 {
        if start_hr > 23 {
            start_hr_exists = true;
            break;
        }

        gen_and_push_impossible_task(
            blocking_slots_map,
            task.clone(),
            start_date.clone(),
            day as u32,
            start_hr as i32,
            hrs_of_the_day[start_hr as usize],
        );
        start_hr = hrs_of_the_day[start_hr as usize];
    }
    if start_hr_exists {
        let mut duration_itr = task.duration;
        let mut start_itr = start_hr;
        let mut ptr = start_hr;
        while ptr <= deadline && start_hr != deadline {
            duration_itr -= 1;
            ptr += 1;
            if hrs_of_the_day[ptr as usize] != -1 || duration_itr == 0 || ptr == deadline {
                let next_available = ptr;
                let range = start_itr as usize..ptr as usize;

                println!("{:#?}", range);
                hrs_of_the_day.splice(
                    range.clone(),
                    repeat(next_available).take((ptr - start_itr) as usize),
                );
                let schedule_of_the_day = scheduled.get_mut(day as usize).unwrap();
                schedule_of_the_day.outputs.push(OutputSlot {
                    goalid: task.goalid.clone(),
                    taskid: task.taskid.clone(),
                    start: format_date((start_date.day() + day as u32 - 1) as i32, start_itr),
                    deadline: format_date((start_date.day() + day as u32 - 1) as i32, ptr),
                    duration: ptr - start_itr,
                    title: task.title.clone(),
                });
                while hrs_of_the_day[ptr as usize] != -1 {
                    if ptr > 32 {
                        break;
                    }
                    let mut task_copy = task.clone();
                    task_copy.duration = duration_itr;
                    gen_and_push_impossible_task(
                        blocking_slots_map,
                        task_copy,
                        start_date.clone(),
                        day as u32,
                        ptr,
                        hrs_of_the_day[ptr as usize],
                    );
                    ptr = hrs_of_the_day[ptr as usize];
                }

                if ptr > 23 {
                    break;
                }
                start_itr = ptr;
                if duration_itr == 0 {
                    break;
                }
            }
        }

        if duration_itr != 0 {
            let goal_due_hrs = due_task_hrs.entry(task.goalid.clone()).or_insert(0);
            *goal_due_hrs += duration_itr;
        }
    }
}
