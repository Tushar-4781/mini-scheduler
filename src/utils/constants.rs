use chrono::{NaiveDate, NaiveDateTime};

pub fn default_on_days() -> Vec<&'static str> {
    vec![
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ]
}
pub fn get_day_name(date: NaiveDateTime) -> String {
    // Format the date to include the day name
    let formatted_date = date.format("%a").to_string();

    // Return the formatted date
    formatted_date
}
