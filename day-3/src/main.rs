use std::collections::HashSet;

fn split_rucksack(rucksack: &str) -> (&str, &str) {
    let capacity = rucksack.len();
    if capacity % 2 != 0 {
        panic!("Compartments are not evenly packed");
    }

    (
        &rucksack[0..capacity / 2],
        &rucksack[capacity / 2..capacity],
    )
}

fn set_from_string(s: &str) -> HashSet<char> {
    let mut h = HashSet::new();
    for c in s.chars() {
        h.insert(c);
    }
    h
}

fn check_rucksack(rucksack: &str) -> Option<char> {
    let (compartment_1, compartment_2) = split_rucksack(rucksack);
    let (unique_items_1, unique_items_2) = (
        set_from_string(compartment_1),
        set_from_string(compartment_2),
    );
    let common: Vec<&char> = unique_items_1.intersection(&unique_items_2).collect();
    match common.len() {
        0 => None,
        1 => Some(*common[0]),
        _ => panic!("More than one common item found"),
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_line() {
        assert_eq!(check_rucksack("vJrwpWtwJgWrhcsFMMfFFhFp"), Some('p'));
    }

    #[test]
    fn check_split_rucksack() {
        assert_eq!(split_rucksack("rucksack"), ("ruck", "sack"));
    }

    #[test]
    #[should_panic]
    fn check_split_uneven_rucksack_panics() {
        split_rucksack("rucksacks");
    }
}
