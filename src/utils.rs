use chrono::{Local, NaiveTime};

#[derive(Clone, PartialEq)]
pub struct Time {
    mock_time: Option<NaiveTime>,
}

impl Time {
    pub fn new(mock_time: Option<NaiveTime>) -> Self {
        Self { mock_time }
    }

    pub fn now(&self) -> NaiveTime {
        match self.mock_time {
            Some(p) => p,
            None => Local::now().time(),
        }
    }
}

pub fn int_in_range(value: i32, min: i32, max: i32) -> i32 {
    if value <= min {
        return min;
    }
    if value >= max {
        return max;
    }

    value
}

pub fn shader_hash_from_path(path: String) -> Option<String> {
    Some(path.split("/").last().unwrap().to_string())
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_time_mock() {
        let time = Time::new(Some(NaiveTime::parse_from_str("12:00", "%H:%M").unwrap()));
        assert_eq!(
            time.now(),
            NaiveTime::parse_from_str("12:00", "%H:%M").unwrap()
        );
        assert!(time.now() > NaiveTime::parse_from_str("11:00", "%H:%M").unwrap());
        assert!(time.now() > NaiveTime::parse_from_str("11:59", "%H:%M").unwrap());
        assert!(time.now() < NaiveTime::parse_from_str("12:59", "%H:%M").unwrap());
    }

    #[test]
    fn test_int_in_range() {
        let tests = [
            (50, 1, 100, 50),
            (50, 1, 50, 50),
            (1, 1, 100, 1),
            (150, 1, 100, 100),
            (0, 1, 100, 1),
        ];
        for (value, min, max, expected) in tests {
            assert_eq!(int_in_range(value, min, max), expected)
        }
    }
}
