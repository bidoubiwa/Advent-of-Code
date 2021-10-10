use std::collections::{HashMap, HashSet};

use day_6::*;
use utils::*;

fn cover_grid_influence(mut grid: Grid, sectors: &Vec<Sector>) -> Grid {
    for sector in sectors {
        for y in 0..=grid.height {
            for x in 0..=grid.width {
                let current_distance =
                    sub_usize(sector.position.x, x) + sub_usize(sector.position.y, y);
                let current_point = Point::new(x, y);

                let area = grid.areas.entry(current_point).or_insert(AreaControl {
                    controlled_by: sector.name.clone(),
                    distance_to_sector: current_distance,
                    position: current_point.clone(),
                });
                if area.distance_to_sector == current_distance {
                    area.distance_to_sector = current_distance;
                    area.controlled_by = ".".to_string();
                } else if area.distance_to_sector > current_distance {
                    area.distance_to_sector = current_distance;
                    area.controlled_by = sector.name.clone();
                }
            }
        }
    }
    return grid;
}

fn get_infinite_sectors(grid: &Grid) -> HashSet<String> {
    let mut infinite_sectors = HashSet::new();

    for y in 0..=grid.height {
        let current_point = Point::new(0, y);
        let sector = &grid.areas[&current_point].controlled_by;
        infinite_sectors.insert(sector.clone());

        let current_point = Point::new(grid.width, y);
        let sector = &grid.areas[&current_point].controlled_by;
        infinite_sectors.insert(sector.clone());
    }

    for x in 0..grid.width {
        let current_point = Point::new(x, 0);
        let sector = &grid.areas[&current_point].controlled_by;
        infinite_sectors.insert(sector.clone());

        let current_point = Point::new(x, grid.height);
        let sector = &grid.areas[&current_point].controlled_by;
        infinite_sectors.insert(sector.clone());
    }

    return infinite_sectors;
}

fn grid_width(sectors: &Vec<Sector>) -> usize {
    sectors
        .iter()
        .map(|sector| sector.position.y)
        .max()
        .unwrap()
}

fn grid_height(sectors: &Vec<Sector>) -> usize {
    sectors
        .iter()
        .map(|sector| sector.position.x)
        .max()
        .unwrap()
}

fn furthest_in_direction(sectors: &Vec<Sector>, accessor: impl Fn(&Sector) -> usize) -> usize {
    sectors.iter().map(accessor).max().unwrap()
}

fn main() {
    let sectors: Vec<Sector> = read_lines_as();
    let height = furthest_in_direction(&sectors, |sector| sector.position.y);
    let width = furthest_in_direction(&sectors, |sector| sector.position.x);
    let grid = Grid {
        width,
        height,
        areas: HashMap::new(),
    };

    let grid = cover_grid_influence(grid, &sectors);
    let infinite_sectors = get_infinite_sectors(&grid);

    let mut sectors_influences: HashMap<String, usize> = sectors
        .iter()
        .map(|sector| (sector.name.clone(), 0))
        .collect();

    for control in grid.areas.into_values() {
        let sector_name = control.controlled_by;
        if !infinite_sectors.contains(&sector_name) && &sector_name != &"." {
            *sectors_influences.get_mut(&sector_name).unwrap() += 1;
        }
    }

    let biggest_sector = sectors_influences
        .iter()
        .max_by_key(|(_, areas_controlled)| *areas_controlled)
        .unwrap();

    println!(
        "Sector:{} controls {} areas",
        biggest_sector.0, biggest_sector.1
    )
}
