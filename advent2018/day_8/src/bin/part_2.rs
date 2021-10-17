use day_8::*;
use utils::*;

fn main() {
    let license = read();
    let license = license.split_whitespace();
    let license: Vec<usize> = license.map(|symbol| symbol.parse().unwrap()).collect();
    let (mut node, _) = parse_node(&license);
    let sum = node.metadata_sum();
    println!("The sum of leaves metadata is {}.", sum)
}
