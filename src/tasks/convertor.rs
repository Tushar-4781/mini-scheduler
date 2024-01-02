use uuid::Uuid;

use crate::{types::{scheduler_input::{SIGoal, SIFilters}, scheduler_output::Slot}, utils::constants::CAL_DAYS};

impl SIFilters {
    // Create a new instance with default values
    fn new_default() -> Self {
        SIFilters {
            after_time: 0,
            before_time: 24,
            on_days: CAL_DAYS,
            not_on: Some(Vec::new()),
        }
    }
} 
pub fn convert_into_task(goal: SIGoal) {
    
    // Create a GoalFilters instance with default values
    let mut default_filters = SIFilters::new_default();

    // If goal.filters is Some, update the fields with provided values
    goal.filters.map_or_else(
        || {}, // Do nothing when goal.filters is None
        |filters| {
            default_filters.after_time = filters.after_time;
            default_filters.before_time = filters.before_time;
            default_filters.on_days = filters.on_days.clone().unwrap_or_default();
            default_filters.not_on = filters.not_on.clone().unwrap_or_default();
        },
    );
    let valid_days: Vec<String> = default_filters
    .on_days
    .iter()
    .cloned()
    .filter(|&day| !filters.not_on.contains(&day))
    .collect();

}