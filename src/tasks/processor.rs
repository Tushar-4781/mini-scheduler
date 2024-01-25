use chrono::NaiveDateTime;
use serde_json::map::OccupiedEntry;
use uuid::Uuid;

use crate::{
    types::{
        scheduler_engine::{TBuffer, TBufferMap, TDueHrsMap},
        scheduler_input::SIGoal,
        scheduler_output::Slot,
    },
    utils::constants::get_day_name,
};

pub fn processBudgetGoal(
    calendar: &mut Vec<Vec<Slot>>,
    buffer_map: &mut TBufferMap,
    due_hrs_map: &mut TDueHrsMap,
    goal: &SIGoal,
    valid_days: Vec<String>,
    min: i32,
    min_duration: i32,
    start_date: NaiveDateTime,
) {
    let tmp_start = start_date;
    let total_duration = min;
    let created_at = goal.created_at.clone();
    let deadline = {
        if goal.deadline.is_some() {
            goal.deadline.clone()
        } else {
            None
        }
    };
    for key in 0..=6 {
        if deadline.is_some() && deadline < Some(tmp_start) {
            break;
        }
        let dayItr = get_day_name(tmp_start);

        if valid_days.contains(&dayItr) {
            calendar[key + 1].push(Slot {
                goalid: goal.id.clone(),
                taskid: Uuid::new_v4(),
                title: goal.title.clone(),
                start: goal.filters.as_ref().unwrap().after_time,
                deadline: goal.filters.as_ref().unwrap().after_time,
                duration: {
                    if min > total_duration {
                        total_duration
                    } else {
                        min
                    }
                },
            });
            if min > total_duration {
                let entry = buffer_map.entry(goal.id.clone());
                // Matching on the Entry enum to handle both cases
                let buffer = match entry {
                    std::collections::hash_map::Entry::Occupied(o) => {
                        let existing_value = o.into_mut();
                        existing_value
                    }
                    std::collections::hash_map::Entry::Vacant(v) => {
                        let new_value = v.insert(Vec::new());
                        new_value
                    }
                };
                buffer.push(TBuffer {
                    next_buffer: key as i32 + 1,
                    available_buffer: min - total_duration,
                })
            }
            if total_duration >= 0 {
                let entry = due_hrs_map.entry(goal.id.clone());
                match entry {
                    std::collections::hash_map::Entry::Occupied(mut occupied) => {
                        // Key exists, increment its value
                        *occupied.get_mut() += total_duration;
                    }
                    std::collections::hash_map::Entry::Vacant(vacant) => {
                        // Key doesn't exist, set its value to 0
                        vacant.insert(total_duration);
                    }
                }
            }
        }
    }
}
