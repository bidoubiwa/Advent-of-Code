use day_7::*;
use std::collections::HashMap;
use utils::*;

fn assign_steps(starting_block: &mut Vec<&Step>, workers: &mut Vec<(i32, char)>) {
    workers.iter_mut().for_each(|(state, step_name)| {
        if *state == -1 {
            if starting_block.len() > 0 {
                *step_name = starting_block[0].name;
                *state = (*step_name as u8 - b'A' + 61) as i32;
                starting_block.remove(0);
            }
        }
    });
}

fn completed_sequence(starting_block: &Vec<&Step>, workers: &Vec<(i32, char)>) -> bool {
    starting_block.len() == 0 && workers.iter().all(|(state, _)| *state == -1)
}

pub fn process_step<'a>(
    current_step: char,
    starting_block: &mut Vec<&'a Step>,
    processed: &mut Vec<char>,
    steps: &'a HashMap<char, Step>,
) {
    let step = steps.get(&current_step).unwrap();
    processed.push(step.name);
    let childs = &step.childs;
    childs.iter().for_each(|child| {
        if fulfilled_predicate(&steps[child], &processed) {
            starting_block.push(&steps[child])
        }
    });
}

fn main() {
    let lines = read_lines();
    let mut steps = create_step_lists();
    populate_steps(&mut steps, lines);

    let mut starting_block = create_starting_block(&steps);
    let mut sequence: Vec<char> = Vec::new();
    let mut workers = vec![(-1, ' '); 5];
    assign_steps(&mut starting_block, &mut workers);

    let mut seconds = 0;
    while !completed_sequence(&starting_block, &workers) {
        workers.iter_mut().for_each(|(state, step_name)| {
            if *state > -1 {
                *state = *state - 1;
            }
            if *state == 0 {
                process_step(*step_name, &mut starting_block, &mut sequence, &steps);
                sort_block(&mut starting_block);
                *state = -1;
            }
        });
        assign_steps(&mut starting_block, &mut workers);
        seconds += 1;
    }
    println!(
        "The complete sequence order is: {} in {} seconds",
        sequence.iter().collect::<String>(),
        seconds
    )
}
