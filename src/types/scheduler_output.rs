use uuid::Uuid;

#[derive(Clone)]
pub struct Slot {
    pub goalid: String, // Replace TypeOfGoalId with the actual type of goal.id
    pub taskid: Uuid,
    pub title: String, // Replace TypeOfGoalTitle with the actual type of goal.title
    pub start: i32, // Replace TypeOfStartTime with the actual type of after_time
    pub deadline: i32, // Replace TypeOfDeadline with the actual type of before_time
    pub duration: i32, // Replace TypeOfDuration with the actual type of total_duration
}