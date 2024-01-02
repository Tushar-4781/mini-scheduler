use chrono::{naive::NaiveDateWeeksIterator, NaiveDate, NaiveDateTime};
use serde::Deserialize;
use std::collections::HashMap;

use crate::utils::constants::CalDay;

#[derive(Debug, Deserialize, Clone)]
pub struct SIFilters {
    pub after_time: u32,
    pub before_time: u32,
    pub on_days: Option<Vec<CalDay>>,
    pub not_on: Option<Vec<CalDay>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SIGoal {
    pub id: String,
    pub title: String,
    pub min_duration: u32,
    pub start: Option<NaiveDateTime>,
    pub deadline: Option<NaiveDateTime>,
    pub filters: Option<SIFilters>,
}
#[derive(Debug, Deserialize)]
pub struct SchedulerInput {
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub goals: HashMap<String, SIGoal>,
}
