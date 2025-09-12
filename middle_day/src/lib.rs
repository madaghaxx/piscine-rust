use chrono::{NaiveDate, Weekday};

pub fn middle_day(year: u32) -> Option<Weekday> {
    let is_leap = (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0);
    let middle_day = if is_leap { 184 } else { 183 };
    let date = NaiveDate::from_yo_opt(year as i32, middle_day)?;
    Some(date.weekday())
}