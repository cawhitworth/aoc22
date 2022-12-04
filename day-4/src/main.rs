mod error;
use error::Error;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type Elf = (u32, u32);

fn outer_contains_inner(outer: Elf, inner: Elf) -> bool {
    let (outer_lower, outer_upper) = outer;
    let (inner_lower, inner_upper) = inner;
    inner_lower >= outer_lower && inner_upper <= outer_upper
}

fn either_contains(range1: Elf, range2: Elf) -> bool {
    outer_contains_inner(range1, range2) || outer_contains_inner(range2, range1)
}

fn overlaps_at_all(range1: Elf, range2: Elf) -> bool {
    let (range1_lower, range1_upper) = range1;
    let (range2_lower, range2_upper) = range2;
    let r1 = range1_lower..=range1_upper;
    let r2 = range2_lower..=range2_upper;
    r1.contains(&range2_lower)
        || r1.contains(&range2_upper)
        || r2.contains(&range1_lower)
        || r2.contains(&range1_upper)
}

fn parse_range(range: &str) -> Result<Elf, Error> {
    let split: Vec<&str> = range.split('-').collect();
    if split.len() != 2 {
        Err(Error::new(format!(
            "Expected two values separated by '-' in {}",
            range
        )))
    } else {
        let lower = split[0].parse::<u32>();
        let upper = split[1].parse::<u32>();

        if let (Ok(r1), Ok(r2)) = (lower, upper) {
            Ok((r1, r2))
        } else {
            Err(Error::new(format!(
                "Could not parse {} as two numbers",
                range
            )))
        }
    }
}

fn parse_line(line: &str) -> Result<(Elf, Elf), Error> {
    let split: Vec<&str> = line.split(',').collect();
    if split.len() != 2 {
        Err(Error::new(format!("Expected two ranges in {}", line)))
    } else {
        Ok((parse_range(split[0])?, parse_range(split[1])?))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = File::open("input")?;
    let mut total_score = 0;
    let mut total_score_2 = 0;
    {
        let reader = BufReader::new(input);
        for line in reader.lines() {
            let line_str = line?;
            let (elf1, elf2) = parse_line(&line_str)?;
            if either_contains(elf1, elf2) {
                total_score += 1;
            }
            if overlaps_at_all(elf1, elf2) {
                total_score_2 += 1;
            }
        }
    }

    println!("Containing elves: {}", total_score);
    println!("Overlapping elves: {}", total_score_2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_outer_containers_inner() {
        let test_data = vec![
            ((2, 4), (6, 8), false),
            ((2, 3), (4, 5), false),
            ((5, 7), (7, 9), false),
            ((2, 8), (3, 7), true),
            ((6, 6), (4, 6), false),
            ((2, 6), (4, 8), false),
        ];

        for (elf1, elf2, contained) in test_data {
            assert_eq!(
                outer_contains_inner(elf1, elf2),
                contained,
                "{:?} {:?} {}",
                elf1,
                elf2,
                contained
            );
        }
    }

    #[test]
    fn check_is_contained() {
        let test_data = vec![
            ((2, 4), (6, 8), false),
            ((2, 3), (4, 5), false),
            ((5, 7), (7, 9), false),
            ((2, 8), (3, 7), true),
            ((6, 6), (4, 6), true),
            ((2, 6), (4, 8), false),
        ];

        for (elf1, elf2, contained) in test_data {
            assert_eq!(either_contains(elf1, elf2), contained);
        }
    }

    #[test]
    fn check_overlap_at_all() {
        let test_data = vec![
            ((2, 4), (6, 8), false),
            ((2, 3), (4, 5), false),
            ((5, 7), (7, 9), true),
            ((2, 8), (3, 7), true),
            ((6, 6), (4, 6), true),
            ((2, 6), (4, 8), true),
        ];

        for (elf1, elf2, overlaps) in test_data {
            assert_eq!(
                overlaps_at_all(elf1, elf2),
                overlaps,
                "{:?} {:?}",
                elf1,
                elf2
            );
        }
    }

    #[test]
    fn test_parse() -> Result<(), Error> {
        let test_data = vec![("2-4,6-8", ((2, 4), (6, 8)))];

        for (line, ranges) in test_data {
            assert_eq!(parse_line(line)?, ranges);
        }

        Ok(())
    }
}
