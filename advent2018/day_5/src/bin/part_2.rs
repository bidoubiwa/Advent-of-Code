use day_5::*;
use utils::*;

fn main() {
    let polymere_sequence: Vec<String> = read_lines();
    let polymere_sequence: String = polymere_sequence.into_iter().next().unwrap();
    let polymere_sequence: Vec<char> = polymere_sequence.chars().collect();

    let mut unit_type_biggest_impact: char = ' ';
    let mut min_polymere_length = usize::MAX;
    for unit_type in 'a'..='z' {
        let polymere_sequence = react_polymere_sequence(
            polymere_sequence
                .clone()
                .into_iter()
                .filter(|unit| unit.to_ascii_lowercase() != unit_type)
                .collect(),
        );

        if polymere_sequence.len() < min_polymere_length {
            min_polymere_length = polymere_sequence.len();
            unit_type_biggest_impact = unit_type as char;
        }
    }

    println!(
        "Removing unit type {}/{} first reduced the final polymere sequence to {} length.",
        unit_type_biggest_impact.to_ascii_lowercase(),
        unit_type_biggest_impact.to_ascii_uppercase(),
        min_polymere_length
    )
}
