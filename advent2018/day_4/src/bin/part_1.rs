use day_4::*;
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
    let reports: Vec<Report> = read_lines_as();
    dbg!(reports.len());
}
