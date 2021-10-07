use day_3::{Claim, Point};
use std::collections::HashMap;
use utils::read_lines;

fn collect_points(claims: Vec<Claim>) -> HashMap<Point, usize> {
    let mut points_map = HashMap::new();
    claims.iter().for_each(|claim| {
        claim
            .points_coverage()
            .into_iter()
            .for_each(|point| *points_map.entry(point).or_insert(0) += 1)
    });
    points_map
}

fn overlapped_fabric_count(fabric_map: HashMap<Point, usize>) -> usize {
    fabric_map.into_values().filter(|val| val > &1).count()
}

fn main() {
    let lines: Vec<String> = read_lines();
    let claims: Vec<Claim> = lines.iter().map(|line| line.parse().unwrap()).collect();
    let fabric_map = collect_points(claims);
    let overlaps = overlapped_fabric_count(fabric_map);
    dbg!(overlaps);
    // dbg!(fabric_map);
}
