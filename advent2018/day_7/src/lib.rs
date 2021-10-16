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
