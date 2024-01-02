use chrono::NaiveDate;
use serde::Deserialize;

pub const CAL_DAYS: Vec<CalDay> = vec![
    CalDay::Mon,
    CalDay::Tue,
    CalDay::Wed,
    CalDay::Thu,
    CalDay::Fri,
    CalDay::Sat,
    CalDay::Sun,
];
#[derive(Debug, Deserialize, Clone)]
pub enum CalDay {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

pub fn get_day(date: NaiveDate) -> CalDay {
    CalDay::Mon
}
