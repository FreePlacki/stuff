//! A module for getting and parsing dates.
use std::fmt::Display;

use chrono::prelude::*;

#[derive(Debug)]
pub struct Date {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
}

impl Date {
    pub fn new(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> Date {
        Date {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }

    pub fn current_date() -> Date {
        let now = Local::now();
        let year = now.year();
        let month = now.month();
        let day = now.day();
        let hour = now.hour();
        let minute = now.minute();
        let second = now.second();

        Date::new(year, month, day, hour, minute, second)
    }

    fn get_date_from_string(date_string: String) -> Date {
        let date_string_split: Vec<&str> = date_string.split(" ").collect();
        let year = date_string_split[0].parse::<i32>().unwrap();
        let month = date_string_split[1].parse::<u32>().unwrap();
        let day = date_string_split[2].parse::<u32>().unwrap();
        let hour = date_string_split[3].parse::<u32>().unwrap();
        let minute = date_string_split[4].parse::<u32>().unwrap();
        let second = date_string_split[5].parse::<u32>().unwrap();

        Date::new(year, month, day, hour, minute, second)
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}-{:02}-{} {:02}:{:02}:{:02}",
            self.day, self.month, self.year, self.hour, self.minute, self.second
        )
    }
}

pub struct Time {
    years: i32,
    months: i32,
    days: i32,
    hours: i32,
    minutes: i32,
    seconds: i32,
    is_past: bool,
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("");
        if self.years != 0 {
            s += &format!("{}y ", self.years);
        }
        if self.months != 0 {
            s += &format!("{}m ", self.months);
        }
        if self.days != 0 {
            s += &format!("{}d ", self.days);
        }
        if self.hours != 0 {
            s += &format!("{}h ", self.hours);
        }
        if self.minutes != 0 {
            s += &format!("{}m ", self.minutes);
        }
        if self.seconds != 0 {
            s += &format!("{}s", self.seconds);
        }

        write!(f, "{}", s)
    }
}
