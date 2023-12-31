use crate::types::scheduler_input::SIGoal;

pub fn split_goal(input_goal: SIGoal) -> Vec<SIGoal> {
    let mut res: Vec<SIGoal> = Vec::new();
    let mut goal = input_goal.clone();

    if let Some(filters) = &goal.filters {
        let is_splitted = filters.after_time > filters.before_time;

        if is_splitted {
            let mut before_midnight_goal = goal.clone();
            before_midnight_goal.filters.as_mut().unwrap().after_time = 0;
            before_midnight_goal.filters.as_mut().unwrap().before_time = 24;
            before_midnight_goal.min_duration = 24 - filters.after_time;
            res.push(before_midnight_goal);

            goal.min_duration -= 24 - filters.after_time;
            goal.filters.as_mut().unwrap().after_time = 0;
        }
    }

    res.push(goal);
    res
}
