use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::{
    tasks::processor::processBudgetGoal,
    types::{
        scheduler_engine::{TBufferMap, TDueHrsMap},
        scheduler_input::{SIFilters, SIGoal},
        scheduler_output::Slot, // scheduler_output::Slot,
    },
    utils::constants::default_on_days,
};

fn extract_filters(goal: &SIGoal) -> SIFilters {
    let default_on_days: Vec<String> = default_on_days().iter().map(|s| s.to_string()).collect();
    goal.filters
        .as_ref()
        .map(|f| SIFilters {
            after_time: f.after_time,
            before_time: f.before_time,
            on_days: f.on_days.clone().or(Some(default_on_days.clone())),
            not_on: f.not_on.clone().or(Some(Vec::new())),
        })
        .unwrap_or_else(|| SIFilters {
            after_time: 8,
            before_time: 18,
            on_days: Some(default_on_days),
            not_on: Some(Vec::new()),
        })
}
pub fn convert_into_task(
    goal: &SIGoal,
    mut calendar: &mut Vec<Vec<Slot>>,
    mut buffer: &mut TBufferMap,
    mut due_task_hrs: &mut TDueHrsMap,
    start_date: &NaiveDateTime,
) {
    // Create a GoalFilters instance with default values
    let filters = extract_filters(&goal);
    let on_days = filters
        .on_days
        .as_ref()
        .map_or_else(|| Vec::new(), |v| v.clone());
    let not_on = filters
        .not_on
        .as_ref()
        .map_or_else(|| Vec::new(), |v| v.clone());

    let valid_days: Vec<String> = on_days
        .iter()
        .filter(|&day| !not_on.contains(&day))
        .cloned()
        .collect();
    if goal
        .filters
        .as_ref()
        .and_then(|f| f.on_days.as_ref())
        .is_some()
        || goal.repeat.is_some()
    {
        if goal.repeat.as_ref().unwrap() == "daily" {
            let task = Slot {
                taskid: Uuid::new_v4(),
                goalid: goal.id.clone(),
                title: goal.title.clone(),
                start: filters.after_time,
                deadline: filters.before_time,
                duration: goal.min_duration,
            };
            for key in 0..=6 {
                calendar[key].push(task.clone());
            }
        }
    } else {
        processBudgetGoal(
            &mut calendar,
            &mut buffer,
            &mut due_task_hrs,
            goal,
            valid_days.clone(),
            goal.min_duration,
            goal.min_duration,
            start_date.clone(),
        );
    }
    println!("{:?}", valid_days);
}
