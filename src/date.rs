//! A module for getting and parsing dates.
use std::fmt::{Display, Formatter};

use chrono::prelude::*;

#[derive(Debug)]
pub struct Date {
    date: DateTime<Local>,
}

impl Date {
    pub fn new(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> Date {
        Date {
            date: Local.ymd(year, month, day).and_hms(hour, minute, second),
        }
    }

    pub fn get_current_date() -> Date {
        let now = Local::now();
        Date { date: now }
    }

    pub fn get_time_left(&self) -> String {
        let now = Local::now();
        let diff = self.date.signed_duration_since(now);

        let s = if diff.num_weeks() != 0 {
            format!("{}w", diff.num_weeks())
        } else if diff.num_days() != 0 {
            format!("{}d", diff.num_days())
        } else if diff.num_hours() != 0 {
            format!("{}h", diff.num_hours())
        } else if diff.num_minutes() != 0 {
            format!("{}m", diff.num_minutes())
        } else if diff.num_seconds() != 0 {
            format!("{}s", diff.num_seconds())
        } else {
            "0s".to_string()
        };

        s
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.date.format("%Y-%m-%d %H:%M:%S"))
    }
}
