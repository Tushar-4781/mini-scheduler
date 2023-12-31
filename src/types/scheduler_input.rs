use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct SIFilters {
    pub after_time: u32,
    pub before_time: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SIGoal {
    pub id: String,
    pub title: String,
    pub min_duration: u32,
    pub start: Option <String>,
    pub deadline: Option <String>,
    pub filters: Option <SIFilters>,
}
#[derive(Debug, Deserialize)]
pub struct SchedulerInput {
    pub start_date: String,
    pub end_date: String,
    pub goals: HashMap<String, SIGoal>,
}
