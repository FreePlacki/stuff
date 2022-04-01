//! A module for getting and parsing dates.
use chrono::prelude::*;

pub type DateFormat = DateTime<Local>;

pub fn get_time_left(date: DateFormat) -> String {
    let now = Local::now();
    let diff = date.signed_duration_since(now);

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
