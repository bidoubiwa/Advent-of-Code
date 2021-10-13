use std::collections::HashSet;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let mut frequencies = HashSet::new();
    let mut frequency: i32 = 0;
    io::stdin().read_to_string(&mut input).unwrap();

    'frequencyCalculation: loop {
        for line in input.lines() {
            frequency += line.parse::<i32>().unwrap();
            if frequencies.insert(frequency) == false {
                println!("Frequency already achieved: {}", frequency);
                break 'frequencyCalculation;
            }
        }
    }
}
