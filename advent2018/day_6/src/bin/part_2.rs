use day_6::*;
use utils::*;

fn main() {
    let sectors: Vec<Point> = read_lines_as();
    let height = sectors.iter().max_by_key(|point| point.y).unwrap().y;
    let width = sectors.iter().max_by_key(|point| point.x).unwrap().x;

    let mut total_safe_regio = 0;
    for y in 0..=height {
        for x in 0..=width {
            let mut total_distances = 0;
            for sector in sectors.iter() {
                total_distances += sub_usize(x, sector.x) + sub_usize(y, sector.y);
            }
            if total_distances < 10000 {
                total_safe_regio += 1;
            }
        }
    }

    println!("The size of the safe regio is {}.", total_safe_regio);
}
