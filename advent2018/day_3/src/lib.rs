use std::num::ParseIntError;
use std::str::FromStr;

// CLAIM
#[derive(Debug)]
pub struct Claim {
    pub id: usize,
    start: Point,
    dimension: Dimension,
}

impl Claim {
    pub fn points_coverage(&self) -> Vec<Point> {
        (0..self.dimension.width)
            .map(|x| {
                (0..self.dimension.height)
                    .map(move |y| Point::new(x + self.start.x, y + self.start.y))
            })
            .flatten()
            .collect()
    }
}

impl FromStr for Claim {
    type Err = ParseIntError;
    fn from_str(claim: &str) -> Result<Self, Self::Err> {
        // #1 @ 338,764: 20x24
        let splitted_str: Vec<&str> = claim.split_whitespace().collect();
        let id: usize = splitted_str[0].replace("#", "").parse()?;
        let point: Point = splitted_str[2].parse()?;
        let dimension: Dimension = splitted_str[3].parse()?;
        let claim: Claim = Claim {
            id,
            start: point,
            dimension,
        };
        Ok(claim)
    }
}

// POINT

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}
impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

impl FromStr for Point {
    type Err = ParseIntError;
    fn from_str(pt: &str) -> Result<Self, Self::Err> {
        let parsed_pt: Vec<usize> = pt
            .replace(':', "")
            .split(',')
            .map(|number| number.parse())
            .collect::<Result<Vec<usize>, Self::Err>>()?;
        let point = Point {
            x: parsed_pt[0],
            y: parsed_pt[1],
        };
        Ok(point)
    }
}

// DIMENSION

#[derive(Debug)]
pub struct Dimension {
    width: usize,
    height: usize,
}

impl FromStr for Dimension {
    type Err = ParseIntError;
    fn from_str(dim: &str) -> Result<Self, Self::Err> {
        let parsed_dim: Vec<usize> = dim
            .split('x')
            .map(|number| number.parse())
            .collect::<Result<Vec<usize>, Self::Err>>()?;
        let dimension = Dimension {
            width: parsed_dim[0],
            height: parsed_dim[1],
        };
        Ok(dimension)
    }
}
