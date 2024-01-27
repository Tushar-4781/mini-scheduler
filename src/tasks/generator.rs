use std::collections::HashMap;

use chrono::{Datelike, NaiveDateTime};

use crate::{
    tasks::splitter::split_goal,
    types::{
        scheduler_engine::{TBlockingSlotsMap, TBufferMap, TDueHrsMap, TFlexibleWeeklyGoals},
        scheduler_input::{SIGoal, SchedulerInput},
        scheduler_output::{OutputSlot, Slot},
    },
    utils::helpers::format_date,
};

use super::convertor::convert_into_task;

pub fn generate_tasks(
    goals: HashMap<String, SIGoal>,
    start_date: NaiveDateTime,
    mut calendar: &mut Vec<Vec<Slot>>,
    mut buffer: &mut TBufferMap,
    mut due_task_hrs: &mut TDueHrsMap,
    blocking_slots: &mut TBlockingSlotsMap,
    mut flexible_weekly_goals: &mut TFlexibleWeeklyGoals,
) {
    for (id, goal) in goals {
        let splitted_goals = split_goal(goal.clone());
        for splitted_goal in splitted_goals {
            convert_into_task(
                &splitted_goal,
                &mut calendar,
                &mut buffer,
                &mut due_task_hrs,
                &mut flexible_weekly_goals,
                &start_date,
            );
        }
        blocking_slots.insert(
            id.clone(),
            vec![
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
            ],
        );
    }
}

pub fn gen_and_push_impossible_task(
    blocking_slots: &mut TBlockingSlotsMap,
    task: Slot,
    start_date: NaiveDateTime,
    day: u32,
    start: i32,
    end: i32,
) {
    let predicted_deadline = start + task.duration;
    let actual_deadline = {
        if task.deadline < predicted_deadline && task.deadline < end {
            task.deadline
        } else if predicted_deadline > end {
            if end > task.deadline {
                task.deadline
            } else {
                end
            }
        } else {
            predicted_deadline
        }
    };
    let goal_blockages = blocking_slots.get_mut(&task.goalid).unwrap();
    let blockages_of_the_day = goal_blockages.get_mut(day as usize).unwrap();
    blockages_of_the_day.push(OutputSlot {
        goalid: task.goalid,
        taskid: task.taskid,
        start: format_date((start_date.day() + day - 1) as i32, start),
        deadline: format_date((start_date.day() + day - 1) as i32, actual_deadline),
        duration: actual_deadline - start,
        title: task.title,
    })
}
