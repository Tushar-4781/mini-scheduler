use chrono::{DateTime, Datelike, Local, NaiveDate, TimeZone, Utc};

pub fn format_date(date: i32, hour: i32) -> String {
    let local: DateTime<Local> = Local::now();

    let formatted_date_time =
        Local
            .ymd(local.year(), local.month(), date.into())
            .and_hms(hour.into(), 0, 0);

    let formatted_string = formatted_date_time.format("%Y-%m-%dT%H:00:00").;
    formatted_string
}
