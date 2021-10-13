use std::error::Error;
use std::str::FromStr;

pub struct Step {
    pub name: char,
    pub dependencies: Vec<Step>,
}

// impl FromStr for Step {
//     type Err = Box<dyn Error>;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {

//     }
// })
