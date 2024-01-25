use uuid::Uuid;

#[derive(Clone)]
pub struct Slot {
    pub goalid: String,
    pub taskid: Uuid,
    pub start: i32,
    pub deadline: i32,
    pub duration: i32,
    pub title: String,
}

pub struct OutputSlot {
    pub goalid: String,
    pub taskid: Uuid,
    pub start: i32,
    pub deadline: String,
    pub duration: String,
    pub title: String,
}
