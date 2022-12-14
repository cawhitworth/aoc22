mod vec2;
mod field;

use anyhow::{anyhow, Result};
use vec2::Vec2;
use field::{Field, Cell};

use std::{fs, cmp::{min, max}};

fn parse_line(l: &str) -> Result<Vec<Vec2>> {
    Ok(l.split("->").map(|s| Vec2::from(s.trim())).collect())
}

fn bounds(v: &Vec<Vec2>) -> Result<(Vec2, Vec2)> {
    let mut itr = v.into_iter();
    let first = itr.next();
    if first.is_none() {
        return Err(anyhow!("Cannot get bounds for no Vec2s"));
    }
    let (mut top_left, mut bottom_right) = (first.unwrap().clone(), first.unwrap().clone());

    for v in itr {
        top_left.x = min(top_left.x, v.x);
        top_left.y = min(top_left.y, v.y);
        bottom_right.x = max(bottom_right.x, v.x);
        bottom_right.y = max(bottom_right.y, v.y);
    }

    Ok((top_left, bottom_right))
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input")?;

    let lines = input.lines();

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::{anyhow, Result};

    const TEST_DATA: &str =
"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test() -> Result<()> {
        let mut l = TEST_DATA.lines();
        let first = l.next().unwrap();
        assert_eq!(parse_line(first)?, vec![ Vec2::new(498,4), Vec2::new(498,6), Vec2::new(496,6) ]);
        Ok(())
    }

    #[test]
    fn test_draw_lines() -> Result<()> {
        let l = "498,4 -> 498,6 -> 496,6";
        let points = parse_line(l)?;
        let (top_left, bottom_right) = bounds(&points)?;
        let mut f = Field::new(top_left, bottom_right);

        f.draw_lines(parse_line(l)?)?;
        assert_eq!(f.get(Vec2::new(498, 4))?, Cell::Rock);
        assert_eq!(f.get(Vec2::new(498, 5))?, Cell::Rock);
        assert_eq!(f.get(Vec2::new(498, 6))?, Cell::Rock);
        assert_eq!(f.get(Vec2::new(497, 6))?, Cell::Rock);
        assert_eq!(f.get(Vec2::new(496, 6))?, Cell::Rock);

        Ok(())

    }
}
