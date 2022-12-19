mod vec2;
use vec2::Vec2;

use anyhow::{anyhow, Result};
use std::{
    cmp::{max, min},
    fs,
};
use regex::Regex;
use itertools::Itertools;

#[derive(PartialEq, Eq, Debug)]
enum Cell {
    Sensor,
    Beacon,
    InRange,
    Empty
}

#[derive(PartialEq, Eq, Debug)]
struct Sensor {
    loc: Vec2,
    beacon: Vec2
}

impl Sensor {
    fn new(loc: Vec2, beacon: Vec2) -> Self {
        Sensor { loc, beacon }
    }

    fn distance(&self) -> usize {
        self.loc.manhattan(&self.beacon)
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input")?;

    let lines = input.lines();

    let sensors = parse(lines)?;
    println!("{}", invalid_spaces_in_line(2000000, &sensors)?);

    Ok(())
}

fn parse_line(line: &str) -> Result<Sensor> {
    let re = Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();
    let captures = re.captures_iter(line).collect_vec();
    if captures.len() != 2 {
        return Err(anyhow!("Expected only two coordinates"));
    }
    Ok(Sensor::new(Vec2::new(captures[0][1].parse()?, captures[0][2].parse()?), 
        Vec2::new(captures[1][1].parse()?, captures[1][2].parse()?)))
}

fn parse<'a, I>(lines: I) -> Result<Vec<Sensor>>
where I: Iterator<Item = &'a str>
{
    lines.map(parse_line).collect::<Result<Vec<_>>>()
}

fn get_cell(pos: Vec2, sensors: &Vec<Sensor>) -> Cell {
    let mut iter = sensors.into_iter();
    loop {
        if let Some(sensor) = iter.next() {
            if sensor.loc == pos {
                return Cell::Sensor;
            }
            if sensor.beacon == pos {
                return Cell::Beacon
            }
            let beacon_distance = sensor.distance();
            let pos_distance = sensor.loc.manhattan(&pos);
            if pos_distance <= beacon_distance {
                return Cell::InRange;
            }
        } else {
            break;
        }
    }

    Cell::Empty
}

fn lr_boundaries(sensors: &Vec<Sensor>) -> (isize, isize) {
    let mut mn = 0;
    let mut mx = 0;
    for s in sensors {
        let l = s.loc.x - s.distance() as isize;
        let r = s.loc.x + s.distance() as isize;
        mn = min(l, mn);
        mx = max(r, mx);
    }
    (mn, mx)
}

fn invalid_spaces_in_line(y: isize, sensors: &Vec<Sensor>) -> Result<usize> {
    let (l,r) = lr_boundaries(sensors);
    let count = (l..r).filter(|x| get_cell(Vec2::new(*x,y), sensors) == Cell::InRange).collect_vec().len();
    Ok(count)
}

fn project_onto(y: isize, sensors: &Vec<Sensor>) -> Result<Vec<(isize, isize)>> {
    let mut result = vec![];
    for s in sensors {
        let d = s.distance();
        let distance_to_projection = (s.loc.y - y).abs() as usize;
        if distance_to_projection > d {
            continue
        }
        let r = (d - distance_to_projection) as isize;
        result.push((s.loc.x - r, s.loc.x + r));
    }
    Ok(result)
}

fn merge(r1: (isize, isize), r2: (isize, isize)) -> Option<(isize, isize)> {

    let (min1, max1) = r1;
    let (min2, max2) = r2;

    if min1 > max1 { return merge((max1, min1), r2); }
    if min2 > max2 { return merge(r1, (max2, min2)); }
    if min1 > min2 { return merge(r2, r1); }

    if max1 < min2 {
        return None
    }

    Some((min1, max(max1,max2)))
}

fn merge_ranges(ranges: Vec<(isize, isize)>) -> Vec<(isize, isize)> {
    let mut result = ranges.clone();
    loop {

    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() -> Result<()> {
        let lines = TEST_DATA.lines().collect_vec();
        let parsed = parse_line(lines[0])?; 
        let expected = Sensor::new(Vec2::new(2,18), Vec2::new(-2, 15));
        assert_eq!(parsed, expected);
        Ok(())
    }

    #[test]
    fn test_empty() -> Result<()>
    {
        let sensors = parse(TEST_DATA.lines())?;
        assert_eq!(get_cell(Vec2::new(-3, 10), &sensors), Cell::Empty);
        assert_eq!(get_cell(Vec2::new(-2, 10), &sensors), Cell::InRange);
        assert_eq!(get_cell(Vec2::new(24, 10), &sensors), Cell::InRange);
        assert_eq!(get_cell(Vec2::new(25, 10), &sensors), Cell::Empty);

        assert_eq!(get_cell(Vec2::new(13, 11), &sensors), Cell::InRange);
        assert_eq!(get_cell(Vec2::new(14, 11), &sensors), Cell::Empty);
        assert_eq!(get_cell(Vec2::new(15, 11), &sensors), Cell::InRange);
        Ok(())
    }

    #[test]
    fn test_boundaries() -> Result<()>
    {
        let sensors = parse(TEST_DATA.lines())?;
        let (l,r) = lr_boundaries(&sensors);
        assert_eq!(l, -8);
        assert_eq!(r, 28);
        Ok(())
    }

    #[test]
    fn test_count() -> Result<()> {
        let sensors = parse(TEST_DATA.lines())?;
        assert_eq!(invalid_spaces_in_line(10, &sensors)?, 26);

        Ok(())
    }

    #[test]
    fn test_project() -> Result<()> {
        let test = Sensor::new(Vec2::new(0, 11), Vec2::new(2, 10));

        let r = project_onto(10, &vec![test])?;
        assert_eq!(r, vec![(-2, 2)]);
        Ok(())
    }

    #[test]
    fn test_merge() {
        assert_eq!(merge((0,5), (3,10)), Some((0,10)));
        assert_eq!(merge((3,10), (0,5)), Some((0,10)));
        assert_eq!(merge((0,5), (7,10)), None);
        assert_eq!(merge((0,5), (3,4)), Some((0,5)));
        assert_eq!(merge((3,4), (0,5)), Some((0,5)));
        assert_eq!(merge((0,5), (5,6)), Some((0,6)));
    }

    static TEST_DATA: &'static str =
"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
}
