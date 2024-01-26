use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime};

pub fn new_date(date: i32, hour: i32) -> NaiveDateTime {
    let local: DateTime<Local> = Local::now();
    let year = local.year();
    let month = local.month();
    let date: NaiveDate = NaiveDate::from_ymd_opt(year, month, date as u32).unwrap();
    let time: NaiveTime = NaiveTime::from_hms_opt(hour as u32, 0, 0).unwrap();
    let date_time: NaiveDateTime = NaiveDateTime::new(date, time);
    date_time
}
