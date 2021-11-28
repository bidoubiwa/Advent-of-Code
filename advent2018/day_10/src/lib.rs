use std::{error, str::FromStr};

#[derive(Debug)]
pub struct Laser {
    pub x: isize,
    pub y: isize,
    pub velocity_x: isize,
    pub velocity_y: isize,
}

impl FromStr for Laser {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<isize> = s
            .split(['<', '>', ','].as_ref())
            .map(str::trim)
            .filter_map(|slice| slice.parse().ok())
            .collect();

        Ok(Laser {
            x: splits[0],
            y: splits[1],
            velocity_x: splits[2],
            velocity_y: splits[3],
        })
    }
}
