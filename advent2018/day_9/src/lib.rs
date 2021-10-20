use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct Game {
    pub players: usize,
    pub marbles: usize,
}

impl FromStr for Game {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let relevant_number: Vec<usize> = s
            .split_whitespace()
            .filter_map(|chunk| chunk.parse().ok())
            .collect();
        let players = relevant_number[0];
        let marbles = relevant_number[1];
        Ok(Game { players, marbles })
    }
}
