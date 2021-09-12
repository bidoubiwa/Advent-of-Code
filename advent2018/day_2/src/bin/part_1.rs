use std::collections::HashMap;
use std::io;
use std::io::Read;

fn scan_box_ids(box_ids: String) -> u16 {
    let mut double_occ_in_box = 0;
    let mut triple_occ_in_box = 0;
    for box_id in box_ids.lines() {
        let mut char_occurencies = HashMap::new();
        for ch in box_id.chars() {
            let counter = char_occurencies.entry(ch).or_insert(0);
            *counter += 1;
        }
        double_occ_in_box += char_occurencies.values().any(|&value| value == 2) as u16;
        triple_occ_in_box += char_occurencies.values().any(|&value| value == 3) as u16;
    }
    double_occ_in_box * triple_occ_in_box
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let checksum: u16 = scan_box_ids(input);
    dbg!(checksum);
}
