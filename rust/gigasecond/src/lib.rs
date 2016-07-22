extern crate chrono;

use chrono::duration::Duration;
use chrono::offset::TimeZone;
use chrono::datetime::DateTime;

pub const GIGASECOND: i64 = 1_000_000_000;

pub fn after<Tz: TimeZone>(date: DateTime<Tz>) -> DateTime<Tz> {
    let datetime = date.checked_add(Duration::seconds(GIGASECOND)).unwrap();
    DateTime::from_utc(datetime.naive_local(), datetime.offset().clone())
}
