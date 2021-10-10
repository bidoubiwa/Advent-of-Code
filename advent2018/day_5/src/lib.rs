pub fn react_polymere_sequence(mut polymere_sequence: Vec<char>) -> Vec<char> {
    let mut units_reacted = true;

    while units_reacted {
        units_reacted = false;
        for (index, units) in polymere_sequence.clone().windows(2).enumerate() {
            let unit_1 = units[0] as i8;
            let unit_2 = units[1] as i8;
            let space = ' ' as i8;
            if (unit_1 - unit_2).abs() == space {
                polymere_sequence.remove(index);
                polymere_sequence.remove(index);
                units_reacted = true;
                break;
            }
        }
    }
    return polymere_sequence;
}
