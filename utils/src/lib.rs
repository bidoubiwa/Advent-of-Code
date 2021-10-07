use std::io;
use std::io::Read;

pub fn read_lines() -> Vec<String> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let collected_lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    return collected_lines;
}