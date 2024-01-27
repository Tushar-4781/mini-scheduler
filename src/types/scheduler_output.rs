use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Slot {
    pub goalid: String,
    pub taskid: String,
    pub start: i32,
    pub deadline: i32,
    pub duration: i32,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OutputSlot {
    pub goalid: String,
    pub taskid: String,
    pub start: String,
    pub deadline: String,
    pub duration: i32,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JSOutputSlots {
    pub day: i32,
    pub outputs: Vec<OutputSlot>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SchedulerOutput {
    pub scheduled: Vec<JSOutputSlots>,
    pub impossible: Vec<JSOutputSlots>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FinalJSOutputSlot {
    pub day: String,
    pub tasks: Vec<OutputSlot>,
}
