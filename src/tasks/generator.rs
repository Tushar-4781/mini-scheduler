use crate::{types::scheduler_input::SchedulerInput, tasks::splitter::split_goal};

use super::convertor::convert_into_task;

pub fn generate_tasks(scheduler_input: SchedulerInput) {
    for (id, goal) in scheduler_input.goals {
        let splitted_goals = split_goal(goal);
        for splitted_goal in splitted_goals {
            convert_into_task(splitted_goal);
        }
    }
}