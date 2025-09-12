use chrono::{NaiveDate, Datelike, Weekday};

pub fn middle_day(year: u32) -> Option<Weekday> {
    let is_leap = (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0);
    if is_leap {
        return None;
    }
    let date = NaiveDate::from_yo_opt(year as i32, 183)?;
    Some(date.weekday())
}