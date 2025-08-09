use chrono::Weekday;
use chrono::{Datelike, NaiveDate};
pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    if year % 4 == 0 {
        return None;
    }
    const MIDDLE: u32 = 183;
    let res = NaiveDate::from_yo_opt(year as i32, MIDDLE).map(|date| date.weekday());
    // println!("middle day: {:#?}", res);
    res
}
