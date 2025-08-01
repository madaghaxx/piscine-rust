use chrono::{ NaiveDate, Datelike };
pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    if year % 4 == 0 {
        return None;
    }
    let middle_date = NaiveDate::from_ymd_opt(year as i32, 1, 1)? + chrono::Duration::days(182);
    Some(middle_date.weekday())
}
