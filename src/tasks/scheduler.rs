use chrono::NaiveDateTime;

use crate::types::scheduler_output::Slot;

pub fn schedule_task(task: Slot, day: i32, start_date: NaiveDateTime, hrs_of_the_day: Vec<i32>) {
    let mut start_hr_exists = true;
    let mut start_hr = task.start as usize;
    while hrs_of_the_day[start_hr] != -1 {
        if start_hr > 23 {
            start_hr_exists = true;
            break;
        }
    }
}
