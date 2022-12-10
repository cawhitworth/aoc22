

use std::{
    collections::HashSet,
    fs::{self},
};

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl Vec2 {
    fn new(x: isize, y: isize) -> Self {
        Vec2 { x, y }
    }

    fn from(dir: (isize, isize)) -> Self {
        let (x, y) = dir;
        Vec2 { x, y }
    }

    fn from_dir(dir: &str) -> Self {
        match dir {
            "U" => Vec2 { x: 0, y: 1 },
            "R" => Vec2 { x: 1, y: 0 },
            "D" => Vec2 { x: 0, y: -1 },
            "L" => Vec2 { x: -1, y: 0 },
            _ => panic!("Invalid direction"),
        }
    }

    fn direction(v1: &Vec2, v2: &Vec2) -> Vec2 {
        Vec2::new((v1.x - v2.x).signum(), (v1.y - v2.y).signum())
    }

    fn add(&self, v: &Vec2) -> Vec2 {
        Vec2::new(self.x + v.x, self.y + v.y)
    }

    fn sub(&self, v: &Vec2) -> Vec2 {
        Vec2::new(self.x - v.x, self.y - v.y)
    }

    fn abs(&self) -> Vec2 {
        Vec2::new(self.x.abs(), self.y.abs())
    }
}

fn tail_pos(head: &Vec2, tail: &Vec2) -> anyhow::Result<Vec2> {
    let dir = Vec2::direction(head, tail);
    let diff = head.sub(tail).abs();

    if (diff.x <= 2 && diff.y == 2) || (diff.y <= 2 && diff.x == 2) {
        Ok(tail.add(&dir))
    } else if diff.x >= 2 && diff.y >= 2 {
        Err(anyhow::anyhow!("{:?} to {:?} is invalid", head, tail))
    } else {
        Ok(*tail)
    }
}

fn parse_line(line: &str) -> anyhow::Result<(usize, Vec2)> {
    let sp = line.split_once(' ');
    if let Some((dir, dist)) = sp {
        return Ok((dist.parse()?, Vec2::from_dir(dir)));
    }

    Err(anyhow::anyhow!("Parse failed: {}", line))
}

fn count_tail_visits<'a, I>(start: &Vec2, length: usize, lines: I) -> anyhow::Result<usize>
where
    I: Iterator<Item = &'a str>,
{
    let mut visits: HashSet<Vec2> = HashSet::new();

    let mut rope = vec![*start; length];
    for line in lines {
        let (distance, direction) = parse_line(line)?;
        for _ in 0..distance {
            rope[0] = rope[0].add(&direction);
            for knot in 1..length {
                rope[knot] = tail_pos(&rope[knot - 1], &rope[knot])?;
            }
            visits.insert(rope[length - 1]);
        }
    }

    Ok(visits.len())
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input")?;

    let head = Vec2::new(0, 0);
    let visits = count_tail_visits(&head, 2, input.lines())?;
    let visits2 = count_tail_visits(&head, 10, input.lines())?;

    println!("Score: {}", visits);
    println!("Score: {}", visits2);

    Ok(())
}

#[cfg(test)]
mod test {
    

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let test_data = vec![
            ((0, 0), (0, 0), (0, 0)),
            ((0, 1), (0, 0), (0, 0)),
            ((0, 2), (0, 0), (0, 1)),
            ((1, 2), (1, 0), (1, 1)),
            ((2, 2), (2, 0), (2, 1)),
            ((3, 2), (2, 0), (3, 1)),
        ];

        for (head, tail, expected) in test_data {
            let hv = Vec2::from(head);
            let tv = Vec2::from(tail);
            let ev = Vec2::from(expected);
            let r = tail_pos(&hv, &tv)?;
            assert_eq!(r, ev, "{:?} {:?} expected {:?}, got {:?}", hv, tv, ev, r);
        }

        Ok(())
    }

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        let test_data = vec![
            ("U 1", (1, (0, 1))),
            ("R 2", (2, (1, 0))),
            ("D 3", (3, (0, -1))),
            ("L 4", (4, (-1, 0))),
        ];

        for (input, (dir, v)) in test_data {
            let ev = Vec2::from(v);
            assert_eq!(parse_line(input)?, (dir, ev));
        }

        Ok(())
    }

    #[test]
    fn test_visit() -> anyhow::Result<()> {
        let test_data = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let head = Vec2::new(0, 0);
        let visits = count_tail_visits(&head, 2, test_data.lines())?;
        assert_eq!(visits, 13);
        Ok(())
    }

    #[test]
    fn test_visit_10() -> anyhow::Result<()> {
        let test_data = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let head = Vec2::new(0, 0);
        let visits = count_tail_visits(&head, 10, test_data.lines())?;
        assert_eq!(visits, 1);
        Ok(())
    }

    #[test]
    fn test_visit_10_2() -> anyhow::Result<()> {
        let test_data = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let head = Vec2::new(0, 0);
        let visits = count_tail_visits(&head, 10, test_data.lines())?;
        assert_eq!(visits, 36);
        Ok(())
    }
}
