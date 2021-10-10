use chrono::prelude::*;
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug)]
pub struct Guard {
    pub id: usize,
    pub sleeping_schedule: [u32; 60],
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum GuardState {
    Sleeping,
    Awake,
    Begin(usize),
}

impl FromStr for GuardState {
    type Err = Box<dyn std::error::Error>;
    fn from_str(log: &str) -> Result<Self, Self::Err> {
        match log {
            log if log.starts_with("Guard") => {
                let guard_id: usize = log.trim_matches(|c: char| !c.is_ascii_digit()).parse()?;
                Ok(Self::Begin(guard_id))
            }
            "wakes up" => Ok(Self::Awake),
            "falls asleep" => Ok(Self::Sleeping),
            log => panic!("unexpected string {}", log),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Report {
    pub date_time: DateTime<Utc>,
    pub guard_state: GuardState,
}

impl Ord for Report {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date_time.cmp(&other.date_time)
    }
}

impl PartialOrd for Report {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Report {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("]");

        let date_time: &str = parts.next().unwrap().trim_start_matches("[");

        let date_time: DateTime<Utc> = Utc.datetime_from_str(date_time, "%Y-%m-%d %H:%M")?;

        let guard_state: GuardState = parts.next().unwrap().trim().parse()?;

        Ok(Report {
            date_time,
            guard_state,
        })
    }
}
