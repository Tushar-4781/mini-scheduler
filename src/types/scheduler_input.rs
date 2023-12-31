use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct SIFilters {
    pub after_time: u32,
    pub before_time: u32,
}

#[derive(Debug, Deserialize)]
pub struct SIGoal {
    pub id: String,
    pub title: String,
    pub min_duration: u32,
    pub start: String,
    pub deadline: String,
    pub filters: SIFilters,
}

#[derive(Debug, Deserialize)]
pub struct SIGoals {
    pub goals: HashMap<String, SIGoal>,
}

#[derive(Debug, Deserialize)]
pub struct SchedulerInput {
    pub start_ate: String,
    pub end_date: String,
    pub goals: SIGoals,
}
