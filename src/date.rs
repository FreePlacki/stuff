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

pub fn date_from_time(time: &str) -> Result<DateFormat, &str> {
    let mut total_time = Local::now();

    for t in time.split(' ') {
        let num = t[0..t.len() - 1].parse::<i64>();
        let num = if let Ok(n) = num {
            n
        } else {
            return Err("Invalid time format");
        };

        let unit = t.chars().last().unwrap();

        total_time = match unit {
            'w' => total_time + chrono::Duration::weeks(num),
            'd' => total_time + chrono::Duration::days(num),
            'h' => total_time + chrono::Duration::hours(num),
            'm' => total_time + chrono::Duration::minutes(num),
            's' => total_time + chrono::Duration::seconds(num),
            _ => return Err("Invalid time format"),
        }
    }
    Ok(total_time)
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    #[test]
    fn test_get_time_left() {
        let date = Local::now() + Duration::weeks(2) + Duration::milliseconds(10);
        assert_eq!(get_time_left(date), "2w");

        let date = Local::now() + Duration::seconds(2) + Duration::milliseconds(10);
        assert_eq!(get_time_left(date), "2s");

        let date = Local::now();
        assert_eq!(get_time_left(date), "0s", "No time left should return 0s");

        let date = Local::now() - (Duration::days(2) + Duration::milliseconds(10));
        assert_eq!(
            get_time_left(date),
            "-2d",
            "Negative time should be formatted correctly"
        );
    }

    #[test]
    fn test_date_from_time() {
        macro_rules! test_date {
            ($time:expr, $duration:expr, $message:expr) => {
                let diff = Local::now() + $duration - date_from_time($time).unwrap();
                assert!(
                    diff < Duration::milliseconds(1) && diff > Duration::milliseconds(-1),
                    $message
                );
            };
        }

        test_date!(
            "2d",
            Duration::days(2),
            "Parsing with one value should work"
        );

        test_date!(
            "2d 1h 5s",
            Duration::days(2) + Duration::hours(1) + Duration::seconds(5),
            "Parsing with multiple values should work"
        );

        test_date!(
            "2h 5w 10s",
            Duration::hours(2) + Duration::weeks(5) + Duration::seconds(10),
            "Parsing with multiple values in random order should work"
        );

        test_date!(
            "3d -5w 23m",
            Duration::days(3) - Duration::weeks(5) + Duration::minutes(23),
            "Parsing with negative values should work"
        );

        let time_str = "2dw 3";
        assert_eq!(
            date_from_time(time_str),
            Err("Invalid time format"),
            "Parsing with invalid values should return an error"
        );
    }
}
