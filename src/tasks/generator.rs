use crate::{
    tasks::splitter::split_goal,
    types::{
        scheduler_engine::{TBlockingSlotsMap, TBufferMap, TDueHrsMap, TFlexibleWeeklyGoals},
        scheduler_input::SchedulerInput,
        scheduler_output::Slot,
    },
};

use super::convertor::convert_into_task;

pub fn generate_tasks(
    scheduler_input: SchedulerInput,
    mut calendar: &mut Vec<Vec<Slot>>,
    mut buffer: &mut TBufferMap,
    mut due_task_hrs: &mut TDueHrsMap,
    blocking_slots: &mut TBlockingSlotsMap,
    mut flexible_weekly_goals: &mut TFlexibleWeeklyGoals,
) {
    for (id, goal) in scheduler_input.goals {
        let splitted_goals = split_goal(goal.clone());
        for splitted_goal in splitted_goals {
            convert_into_task(
                &splitted_goal,
                &mut calendar,
                &mut buffer,
                &mut due_task_hrs,
                &mut flexible_weekly_goals,
                &scheduler_input.start_date,
            );
        }
        blocking_slots.insert(
            id.clone(),
            vec![
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
            ],
        );
    }
}
