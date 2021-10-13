use day_3::{Claim, Point};
use std::collections::HashMap;
use utils::read_lines;

type PointsMap = HashMap<Point, (usize, Vec<usize>)>;

fn collect_points(claims: Vec<Claim>) -> PointsMap {
    let mut points_map = HashMap::new();
    claims.iter().for_each(|claim| {
        claim.points_coverage().into_iter().for_each(|point| {
            let entry = points_map.entry(point).or_insert((0, vec![]));
            entry.0 += 1;
            entry.1.push(claim.id);
        })
    });
    points_map
}

fn find_fabric_outcast(points_map: PointsMap) -> usize {
    let mut claims_overlaps = HashMap::new();
    points_map
        .into_values()
        .for_each(|(overlap_count, claim_ids)| {
            claim_ids.into_iter().for_each(|id| {
                let entry = claims_overlaps.entry(id).or_insert(0);
                if overlap_count > 1 {
                    *entry += 1;
                }
            })
        });
    claims_overlaps
        .into_iter()
        .find(|(_, fabric_overlaps)| fabric_overlaps <= &1)
        .unwrap()
        .0
}

fn main() {
    let lines: Vec<String> = read_lines();
    let claims: Vec<Claim> = lines.iter().map(|line| line.parse().unwrap()).collect();
    let fabric_map = collect_points(claims);
    let fabric_outcast_id = find_fabric_outcast(fabric_map);
    dbg!(fabric_outcast_id);
}
