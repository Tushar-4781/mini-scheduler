use chrono::NaiveDateTime;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub enum BudgetType {
    Daily,
    Weekly,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SIBudget {
    pub budget_type: BudgetType,
    pub min: i32,
}

#[derive(Debug, Deserialize, Clone)]

pub struct SIFilters {
    pub after_time: i32,
    pub before_time: i32,
    pub on_days: Option<Vec<String>>,
    pub not_on: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SIGoal {
    pub id: String,
    pub title: String,
    pub min_duration: i32,
    pub start: Option<NaiveDateTime>,
    pub deadline: Option<NaiveDateTime>,
    pub repeat: Option<String>,
    pub filters: Option<SIFilters>,
    pub created_at: NaiveDateTime,
    pub budgets: Option<Vec<SIBudget>>,
    pub children: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SchedulerInput {
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub goals: HashMap<String, SIGoal>,
}
