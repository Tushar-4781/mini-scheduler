use std::collections::HashMap;

use super::scheduler_output::Slot;

pub struct FlexibleWeeklyGoal {
    slot: Slot,
    valid_days: Vec<String>,
}

pub struct TBuffer {
    pub next_buffer: i32,
    pub available_buffer: i32,
}

pub type TBufferMap = HashMap<String, Vec<TBuffer>>;

pub type TDueHrsMap = HashMap<String, i32>;
