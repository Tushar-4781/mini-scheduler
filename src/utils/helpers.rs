use std::collections::HashMap;

use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime};

use crate::types::scheduler_input::SIGoal;

pub fn format_date(date: i32, hour: i32) -> String {
    let local: DateTime<Local> = Local::now();
    let year = local.year();
    let month = local.month();
    let date: NaiveDate = NaiveDate::from_ymd_opt(year, month, date as u32).unwrap();
    let time: NaiveTime = NaiveTime::from_hms_opt(hour as u32, 0, 0).unwrap();
    let date_time: NaiveDateTime = NaiveDateTime::new(date, time);
    date_time.to_string()
}

pub fn traverse_the_tree(
    id: String,
    goals: &HashMap<String, SIGoal>,
    solo_goals: &mut HashMap<String, SIGoal>,
    visited: &mut Vec<String>,
) -> i32 {
    let mut goal = goals.get(&id.clone()).unwrap().clone();
    visited.push(goal.id.clone());
    let parent_goal = goals.get(&goal.id.clone()).unwrap();
    let mut parent_goal_duration = parent_goal.min_duration;
    if parent_goal.children.is_none() {
        goal.min_duration = parent_goal_duration;
        solo_goals.insert(goal.id.clone(), goal);
        return parent_goal_duration;
    } else {
        for child in parent_goal.children.clone().unwrap().iter() {
            let occupied = traverse_the_tree(child.clone(), goals, solo_goals, visited);
            if parent_goal_duration > 0 {
                parent_goal_duration -= occupied;
            }
        }
        if parent_goal_duration > 0 {
            goal.title = parent_goal.title.clone() + "filler";
            goal.min_duration = parent_goal_duration;
        }
    }
    return parent_goal.min_duration;
}

pub fn break_the_tree(goals: HashMap<String, SIGoal>) -> HashMap<std::string::String, SIGoal> {
    let mut visited: Vec<String> = Vec::new();
    let mut solo_goals: HashMap<String, SIGoal> = HashMap::new();

    for (key, value) in goals.iter() {
        if value.children.is_some() {
            traverse_the_tree(key.clone(), &goals, &mut solo_goals, &mut visited);
        }
    }
    for (key, value) in goals.iter() {
        if !visited.contains(&key.clone()) {
            solo_goals.insert(key.clone(), value.clone());
        }
    }

    return solo_goals;
}
