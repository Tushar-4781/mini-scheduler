use uuid::Uuid;

use crate::{
    types::{
        scheduler_input::{SIFilters, SIGoal},
        scheduler_output::Slot,
    },
    utils::constants::{CalDay, CAL_DAYS},
};

impl SIFilters {
    // Create a new instance with default values
    fn new_default() -> Self {
        SIFilters {
            after_time: 0,
            before_time: 24,
            on_days: Some(CAL_DAYS),
            not_on: Some(Vec::new()),
        }
    }
}
pub fn convert_into_task(goal: SIGoal) {
    // Create a GoalFilters instance with default values
    let default_filters = SIFilters::new_default();
    let mut filters = SIFilters::new_default();
    match goal.filters {
        Some(user_filters) => {
            if user_filters.on_days.is_none() {
                filters.on_days = default_filters.on_days;
            }
            if user_filters.not_on.is_none() {
                filters.not_on = default_filters.not_on;
            }
        }
        None => filters = default_filters,
    }
    let valid_days: Vec<CalDay> = filters
    .on_days .iter()
    .cloned()
    .filter(|&day| !filters.not_on.contains(&day))
    .collect();
}
