use std::collections::HashMap;

use day_7::*;
use utils::*;

pub fn process_step<'a>(
    starting_block: &mut Vec<&'a Step>,
    processed: &mut Vec<char>,
    steps: &'a HashMap<char, Step>,
) {
    let step = starting_block[0];
    processed.push(step.name);
    let childs = &step.childs;
    childs.iter().for_each(|child| {
        if fulfilled_predicate(&steps[child], &processed) {
            starting_block.push(&steps[child])
        }
    });

    starting_block.remove(0);
    sort_block(starting_block);
}

fn main() {
    let lines = read_lines();
    let mut steps = create_step_lists();
    populate_steps(&mut steps, lines);

    let mut starting_block = create_starting_block(&steps);
    let mut sequence = Vec::new();

    while starting_block.len() != 0 {
        process_step(&mut starting_block, &mut sequence, &steps);
    }
    println!(
        "The complete sequence order is: {}",
        sequence.iter().collect::<String>()
    )
}
