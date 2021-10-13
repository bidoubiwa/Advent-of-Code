use std::io;
use std::io::Read;

fn box_ids_diff(current_box: &str, comparing_box: &str) -> Option<String> {
    let mut nbr_diffs = 0;
    let mut diff_index = 0;
    for (index, (current_char, comparing_char)) in
        current_box.chars().zip(comparing_box.chars()).enumerate()
    {
        if current_char != comparing_char {
            nbr_diffs += 1;
            diff_index = index;
        }
        if nbr_diffs > 1 {
            return None;
        }
    }
    let mut similarities = current_box.to_string();
    similarities.remove(diff_index);
    return Some(similarities);
}

fn scan_box_ids(box_ids: String) -> String {
    for (global_index, current_box) in box_ids.lines().enumerate() {
        for comparing_box in box_ids.lines().skip(global_index + 1) {
            match box_ids_diff(current_box, comparing_box) {
                Some(similarities) => return similarities,
                None => (),
            };
        }
    }
    unreachable!()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let correct_box_id: String = scan_box_ids(input);
    dbg!(correct_box_id);
}
