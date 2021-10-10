use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub areas: HashMap<Point, AreaControl>,
}

#[derive(Debug)]
pub struct Sector {
    pub name: String,
    pub position: Point,
    pub controlled_areas: usize,
}

impl FromStr for Sector {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(", ").collect();

        let x_fromstr = coords[0].parse()?;
        let y_fromstr = coords[1].parse()?;

        Ok(Sector {
            name: format!("{}_{}", x_fromstr, y_fromstr),
            position: Point::new(x_fromstr, y_fromstr),
            controlled_areas: 0,
        })
    }
}

#[derive(Debug)]
pub struct AreaControl {
    pub controlled_by: String,
    pub distance_to_sector: usize,
    pub position: Point,
}
