mod field;
mod vec2;

use anyhow::{anyhow, Result};
use field::{Cell, Field};
use vec2::Vec2;

use std::{
    cmp::{max, min},
    fs,
};

fn parse_line(l: &str) -> Result<Vec<Vec2>> {
    Ok(l.split("->").map(|s| Vec2::from(s.trim())).collect())
}

fn bounds(v: &[Vec2]) -> Result<(Vec2, Vec2)> {
    let mut itr = v.iter();
    let first = itr.next();
    if first.is_none() {
        return Err(anyhow!("Cannot get bounds for no Vec2s"));
    }
    let (mut top_left, mut bottom_right) = (*first.unwrap(), *first.unwrap());

    for v in itr {
        top_left.x = min(top_left.x, v.x);
        top_left.y = min(top_left.y, v.y);
        bottom_right.x = max(bottom_right.x, v.x);
        bottom_right.y = max(bottom_right.y, v.y);
    }

    Ok((top_left, bottom_right))
}

#[derive(PartialEq, Eq)]
enum DropResult {
    Okay,
    OutOfBounds,
    Full,
}

fn drop_sand(field: &mut Field, start: Vec2) -> Result<DropResult> {
    if field.get(start)? != Cell::Empty {
        return Ok(DropResult::Full);
    }
    let mut pos = start;

    let to_check = [Vec2::new(0, 1), Vec2::new(-1, 1), Vec2::new(1, 1)];

    loop {
        let mut new_pos = pos;

        for p in to_check.into_iter().map(|d| pos.add(&d)) {
            if !field.in_bounds(p) {
                return Ok(DropResult::OutOfBounds);
            }
            let c = field.get(p)?;
            if c == Cell::Empty {
                new_pos = p;
                break;
            }
        }

        if pos != new_pos {
            pos = new_pos;
        } else {
            break;
        }
    }

    field.put(pos, Cell::Sand)?;

    Ok(DropResult::Okay)
}

fn parse_lines<'a, I>(l: I, with_baseline: bool) -> Result<Field>
where
    I: Iterator<Item = &'a str>,
{
    let mut lines = l.map(parse_line).collect::<Result<Vec<_>>>()?;

    let (mut top_left, mut bottom_right) =
        bounds(&lines.clone().into_iter().flatten().collect::<Vec<_>>())?;

    top_left.y = min(top_left.y, 0);

    if with_baseline {
        top_left.x -= 200;
        bottom_right.x += 200;
        bottom_right.y += 2;
        lines.push(vec![
            Vec2::new(top_left.x, bottom_right.y),
            Vec2::new(bottom_right.x, bottom_right.y),
        ]);
    }

    let mut field = Field::new(top_left, bottom_right);

    for line in lines {
        field.draw_lines(&line)?;
    }

    Ok(field)
}

fn part1(input: &str) -> Result<()> {
    let lines = input.lines();
    let mut field = parse_lines(lines, false)?;

    let mut i = 0;
    loop {
        i += 1;
        let r = drop_sand(&mut field, Vec2::new(500, 0))?;
        if r == DropResult::OutOfBounds {
            break;
        }
    }

    // println!("{}", field);
    println!("{}", i - 1);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let lines = input.lines();
    let mut field = parse_lines(lines, true)?;

    let mut i = 0;
    loop {
        i += 1;
        let r = drop_sand(&mut field, Vec2::new(500, 0))?;
        match r {
            DropResult::Okay => {}
            DropResult::OutOfBounds => {
                println!("{}", field);
                return Err(anyhow!("Should never get out of bounds"));
            }
            DropResult::Full => break,
        }
    }

    println!("{}", field);
    println!("{}", i - 1);

    Ok(())
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;

    const TEST_DATA: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test() -> Result<()> {
        let mut l = TEST_DATA.lines();
        let first = l.next().unwrap();
        assert_eq!(
            parse_line(first)?,
            vec![Vec2::new(498, 4), Vec2::new(498, 6), Vec2::new(496, 6)]
        );
        Ok(())
    }

    #[test]
    fn test_draw_lines() -> Result<()> {
        let l = "498,4 -> 498,6 -> 496,6";
        let points = parse_line(l)?;
        let (top_left, bottom_right) = bounds(&points)?;
        let mut f = Field::new(top_left, bottom_right);

        f.draw_lines(&parse_line(l)?)?;
        assert_eq!(f.get(Vec2::new(498, 4))?, Cell::Rock);
        assert_eq!(f.get(Vec2::new(498, 5))?, Cell::Rock);
        assert_eq!(f.get(Vec2::new(498, 6))?, Cell::Rock);
        assert_eq!(f.get(Vec2::new(497, 6))?, Cell::Rock);
        assert_eq!(f.get(Vec2::new(496, 6))?, Cell::Rock);

        Ok(())
    }

    #[test]
    fn test_drop() -> Result<()> {
        let l = TEST_DATA.lines();
        let mut field = parse_lines(l, false)?;

        drop_sand(&mut field, Vec2::new(500, 0))?;
        drop_sand(&mut field, Vec2::new(500, 0))?;
        drop_sand(&mut field, Vec2::new(500, 0))?;
        drop_sand(&mut field, Vec2::new(500, 0))?;
        drop_sand(&mut field, Vec2::new(500, 0))?;

        assert_eq!(field.get(Vec2::new(500, 8))?, Cell::Sand);
        assert_eq!(field.get(Vec2::new(499, 8))?, Cell::Sand);
        assert_eq!(field.get(Vec2::new(501, 8))?, Cell::Sand);
        assert_eq!(field.get(Vec2::new(500, 7))?, Cell::Sand);
        assert_eq!(field.get(Vec2::new(498, 8))?, Cell::Sand);
        Ok(())
    }

    #[test]
    fn test_drop_lots() -> Result<()> {
        let l = TEST_DATA.lines();
        let mut field = parse_lines(l, true)?;
        for i in 0..100 {
            let r = drop_sand(&mut field, Vec2::new(500, 0))?;
            match r {
                DropResult::Okay => assert!(i < 93),
                DropResult::OutOfBounds => assert!(false),
                DropResult::Full => assert!(i >= 93),
            }
        }

        println!("{}", field);

        Ok(())
    }
}
