use std::fmt::Debug;
use std::io;
use std::io::Read;
use std::str::FromStr;

pub fn read_lines() -> Vec<String> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let collected_lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    return collected_lines;
}

pub fn read_lines_as<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let lines: Vec<String> = read_lines();
    lines.iter().map(|line| line.parse().unwrap()).collect()
}
