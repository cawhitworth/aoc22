use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

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

fn find_common_in_group(rucksacks: &[String]) -> char {
    let sets: Vec<HashSet<char>> = rucksacks.iter().map(|r| set_from_string(r)).collect();

    let mut s = sets[0].clone();
    for item in sets.iter().skip(1) {
        let i = s.intersection(item).cloned();
        s = i.collect::<HashSet<char>>();
    }

    if s.len() != 1 {
        panic!("Expected only one mutual intersection, found {}", s.len())
    } else {
        let v = s.iter().collect::<Vec<_>>();
        *v[0]
    }
}

fn score(c: &char) -> u32 {
    if !c.is_ascii() {
        panic!("Cannot score non-ASCII characters")
    }
    if !c.is_ascii_alphabetic() {
        panic!("Cannot score non-Alphabetic characters")
    }
    let n = *c as u8;
    if (b'a'..=b'z').contains(&n) {
        1 + (n - b'a') as u32
    } else {
        27 + (n - b'A') as u32
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = File::open("input")?;
    let mut total_score = 0;
    let mut total_score_2 = 0;
    {
        let mut group: Vec<String> = Vec::new();
        let reader = BufReader::new(input);
        for line in reader.lines() {
            let str_line = line?;
            if let Some(duplicate) = check_rucksack(&str_line) {
                total_score += score(&duplicate);
            }
            group.push(str_line.clone());
            if group.len() == 3 {
                let common = find_common_in_group(&group);
                total_score_2 += score(&common);
                group.clear();
            }
        }
    }

    println!("{}", total_score);
    println!("{}", total_score_2);
    Ok(())
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

    #[test]
    fn check_score() {
        let test_cases = [('a', 1), ('z', 26), ('A', 27), ('Z', 52)];
        for (c, s) in test_cases {
            assert_eq!(score(&c), s, "{} => {}", c, s);
        }
    }

    #[test]
    fn check_test_data() {
        let test_data = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];

        let mut total_score = 0;
        for line in test_data {
            if let Some(duplicate) = check_rucksack(&line) {
                total_score += score(&duplicate);
            }
        }

        assert_eq!(total_score, 157);
    }

    #[test]
    fn check_find_common_in_group() {
        let group: Vec<String> = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
        ];

        let common = find_common_in_group(&group);

        assert_eq!(common, 'r');
    }
}
