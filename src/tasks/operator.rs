use chrono::NaiveDateTime;

use crate::types::{
    scheduler_engine::{TBlockingSlotsMap, TBufferMap, TDueHrsMap},
    scheduler_output::{JSOutputSlots, Slot},
};

use super::scheduler::schedule_task;

pub fn task_operator(
    day: usize,
    mut hrs_of_the_day: &mut Vec<i32>,
    calendar: &mut Vec<Vec<Slot>>,
    mut due_task_hrs: &mut TDueHrsMap,
    buffer_map: &mut TBufferMap,
    start_date: NaiveDateTime,
    mut scheduled: &mut Vec<JSOutputSlots>,
    mut blocking_slots_map: &mut TBlockingSlotsMap,
) {
    let schedule_of_the_day = calendar.get_mut(day).unwrap();

    schedule_of_the_day.sort_by(|a, b| {
        let compare_deadline_start = (a.deadline - a.start).cmp(&(b.deadline - b.start));
        match compare_deadline_start {
            std::cmp::Ordering::Equal => {
                let compare_duration = a.duration.cmp(&b.duration);
                match compare_duration {
                    std::cmp::Ordering::Equal => {
                        let compare_start = a.start.cmp(&b.start);
                        match compare_start {
                            std::cmp::Ordering::Equal => a.deadline.cmp(&b.deadline),
                            _ => compare_start,
                        }
                    }
                    _ => compare_duration,
                }
            }
            _ => compare_deadline_start,
        }
    });
    for slot in calendar.get(day).unwrap_or(&vec![]) {
        let mut duration = slot.duration;
        // if past_due.is_some() && past_due.unwrap() > &mut 0 {
        if let Some(past_due) = due_task_hrs.get_mut(&slot.goalid) {
            if past_due > &mut 0 {
                if let Some(current_buffer) = buffer_map.get_mut(&slot.goalid) {
                    if current_buffer.len() > 0 && current_buffer[0].next_buffer == day as i32 {
                        let mut buffer_for_this_day = current_buffer[0].available_buffer;
                        if past_due >= &mut buffer_for_this_day {
                            duration = duration + buffer_for_this_day;
                            *past_due -= buffer_for_this_day;
                        } else {
                            duration += *past_due as i32;
                            current_buffer[0].available_buffer =
                                buffer_for_this_day - *past_due as i32;
                            *past_due = 0;
                        }
                    }
                }
            }
        }
        if let Some(current_buffer) = buffer_map.get_mut(&slot.goalid) {
            if current_buffer.len() > 0 && current_buffer[0].next_buffer == day as i32 {
                current_buffer.remove(0);
            }
        }
        if slot.duration > 0 {
            let mut slot_copy = slot.clone();
            slot_copy.duration = duration;
            schedule_task(
                slot_copy,
                day as i32,
                start_date.clone(),
                &mut hrs_of_the_day,
                &mut due_task_hrs,
                &mut blocking_slots_map,
                &mut scheduled,
            )
        }
    }
}
