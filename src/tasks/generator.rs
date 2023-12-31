use crate::{types::scheduler_input::SchedulerInput, tasks::splitter::split_goal};

pub fn generate_tasks(scheduler_input: SchedulerInput) {
    for (id, goal) in scheduler_input.goals {
        let splitted_goals = split_goal(goal);
    }
}