use std::collections::HashMap;

use super::scheduler_output::{OutputSlot, Slot};

pub struct FlexibleWeeklyGoal {
    pub slot: Slot,
    pub valid_days: Vec<String>,
}

pub struct TBuffer {
    pub next_buffer: i32,
    pub available_buffer: i32,
}

pub type TBlockingSlot = Vec<Vec<OutputSlot>>;

pub type TBufferMap = HashMap<String, Vec<TBuffer>>;

pub type TDueHrsMap = HashMap<String, i32>;

pub type TBlockingSlotsMap = HashMap<String, TBlockingSlot>;

pub type TFlexibleWeeklyGoals = Vec<FlexibleWeeklyGoal>;
