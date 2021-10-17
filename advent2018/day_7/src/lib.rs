use std::collections::HashMap;

#[derive(Debug)]
pub struct Step {
    pub name: char,
    pub childs: Vec<char>,
    pub parents: Vec<char>,
}

impl Step {
    pub fn new(name: char) -> Step {
        Step {
            name: name,
            childs: Vec::new(),
            parents: Vec::new(),
        }
    }
}

pub fn populate_steps(steps: &mut HashMap<char, Step>, instructions: Vec<String>) {
    instructions.iter().for_each(|instruction| {
        let relationship: Vec<&str> = instruction
            .split(' ')
            .filter(|slice| slice.len() == 1)
            .collect();
        let parent = relationship[0].chars().next().unwrap();
        let child = relationship[1].chars().next().unwrap();
        steps.get_mut(&child).unwrap().parents.push(parent);
        steps.get_mut(&parent).unwrap().childs.push(child);
    })
}

pub fn sort_block(block: &mut Vec<&Step>) {
    block.sort_by_key(|step| step.name);
}

pub fn fulfilled_predicate(step: &Step, processed: &Vec<char>) -> bool {
    let parents = &step.parents;
    let unfinished: Vec<_> = parents
        .iter()
        .filter(|step| !processed.contains(step))
        .collect();
    unfinished.is_empty()
}

pub fn create_starting_block(steps: &HashMap<char, Step>) -> Vec<&Step> {
    let mut starting_block = steps
        .values()
        .filter(|step| step.parents.is_empty())
        .collect();
    sort_block(&mut starting_block);
    return starting_block;
}

pub fn create_step_lists() -> HashMap<char, Step> {
    let mut steps = HashMap::new();
    ('A'..='Z').for_each(|letter| {
        steps.insert(letter, Step::new(letter));
    });
    return steps;
}
