use day_8::*;
use utils::*;

fn parse_node(mut license: &[usize]) -> (Node, &[usize]) {
    let nbr_of_child = license[0];
    let nbr_of_meta = license[1];

    let mut node = Node::new();
    license = &license[2..];
    for _ in 0..nbr_of_child {
        let (child, new_license) = parse_node(license);
        license = new_license;
        node.childs.push(child);
    }
    for index in 0..nbr_of_meta {
        node.metadata.push(license[index]);
    }
    return (node, &license[(nbr_of_meta)..]);
}

fn main() {
    let license = read();
    let license = license.split_whitespace();
    let license: Vec<usize> = license.map(|symbol| symbol.parse().unwrap()).collect();
    let (node, _) = parse_node(&license);
    let sum = node.raw_metadata_sum();
    println!("The sum of all metadatas is {}.", sum)
}
