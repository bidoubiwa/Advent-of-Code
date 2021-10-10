use chrono::Timelike;
use day_4::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;
use utils::*;

fn read_lines_as<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let lines: Vec<String> = read_lines();
    lines.iter().map(|line| line.parse().unwrap()).collect()
}

fn main() {
    let mut reports: Vec<Report> = read_lines_as();
    let mut guard_logs: Vec<Guard> = Vec::new();
    dbg!(reports.len());
    reports.sort();

    let mut current_guard = match reports[0].guard_state {
        GuardState::Begin(id) => Guard {
            id,
            sleeping_schedule: [0; 60],
        },
        _ => panic!("Did not begin with Begine"),
    };

    let mut started_sleeping = 0;

    for log in &reports {
        match log.guard_state {
            GuardState::Begin(id) => {
                guard_logs.push(current_guard);
                current_guard = Guard {
                    id,
                    sleeping_schedule: [0; 60],
                }
            }
            GuardState::Sleeping => started_sleeping = log.date_time.minute(),
            GuardState::Awake => {
                let awakened = log.date_time.minute();
                (started_sleeping..awakened).for_each(|minute| {
                    current_guard.sleeping_schedule[minute as usize] += 1;
                })
            }
        }
    }

    let mut guards = HashMap::new();

    guard_logs.iter().for_each(|guard_log| {
        let sleep_schedule = guards.entry(guard_log.id).or_insert([0; 60]);
        sleep_schedule
            .iter_mut()
            .zip(guard_log.sleeping_schedule)
            .for_each(|(total_minutes, current_minutes)| *total_minutes += current_minutes);
    });
    dbg!(&guard_logs.len());
    dbg!(&guards);

    let mut max_nbr_sleep_in_min = 0;
    let mut max_minutes = 0;
    let mut biggest_sleeper = 0;
    for (id, sleeping_schedule) in guards {
        let (minute, nbr_sleep_in_min) = sleeping_schedule
            .iter()
            .enumerate()
            .max_by_key(|(_, minute)| *minute)
            .unwrap();
        if *nbr_sleep_in_min > max_nbr_sleep_in_min {
            max_nbr_sleep_in_min = *nbr_sleep_in_min;
            max_minutes = minute;

            biggest_sleeper = id;
        }
    }

    dbg!(max_minutes);
    dbg!(biggest_sleeper);
    dbg!(max_nbr_sleep_in_min);
}
