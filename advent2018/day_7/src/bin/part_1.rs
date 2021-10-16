use day_7::*;
use std::collections::HashMap;
use utils::*;

fn sort_block(block: &mut Vec<&Step>) {
    block.sort_by_key(|step| step.name);
}

fn fulfilled_predicate(step: &Step, processed: &Vec<char>) -> bool {
    let parents = &step.parents;
    let unfinished: Vec<_> = parents
        .iter()
        .filter(|step| !processed.contains(step))
        .collect();
    unfinished.is_empty()
}

fn main() {
    let lines = read_lines();
    let mut steps = HashMap::new();
    ('A'..='Z').for_each(|letter| {
        steps.insert(letter, Step::new(letter));
    });
    populate_steps(&mut steps, lines);
    let mut starting_block: Vec<_> = steps
        .values()
        .filter(|step| step.parents.is_empty())
        .collect();
    sort_block(&mut starting_block);
    let mut sequence = Vec::new();
    loop {
        if starting_block.len() == 0 {
            break;
        }
        let step = starting_block[0];
        sequence.push(step.name);
        let childs = &step.childs;
        childs.iter().for_each(|child| {
            if fulfilled_predicate(&steps[child], &sequence) {
                starting_block.push(&steps[child])
            }
        });
        starting_block.remove(0);
        sort_block(&mut starting_block);
    }
    println!(
        "The complete sequence order is: {}",
        sequence.iter().collect::<String>()
    )
}
