use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

mod vec2;
use vec2::Vec2;

#[derive(Clone, Debug)]
enum NodeType {
    Start,
    End,
    Step(usize),
}

struct Map {
    nodes: Vec<NodeType>,
    size: Vec2,
    start: Vec2,
    end: Vec2,
}

impl Map {
    fn parse<'a, I>(lines: I) -> anyhow::Result<Map>
    where
        I: Iterator<Item = &'a str>,
    {
        let mut nodes = Vec::new();
        let mut size = Vec2::origin();
        let mut start = Vec2::origin();
        let mut end = Vec2::origin();
        for line in lines {
            size.x = line.len() as isize;
            let chars = line.chars();
            for (x, c) in chars.enumerate() {
                let node_type = match c {
                    'S' => {
                        start = Vec2::from((x as isize, size.y));
                        NodeType::Start
                    }
                    'E' => {
                        end = Vec2::from((x as isize, size.y));
                        NodeType::End
                    }
                    i => {
                        if i.is_ascii_lowercase() {
                            NodeType::Step(i as usize - 'a' as usize)
                        } else {
                            return Err(anyhow::anyhow!("Invalid node: {}", i));
                        }
                    }
                };
                nodes.push(node_type);
            }
            size.y += 1;
        }
        Ok(Map {
            nodes,
            size,
            start,
            end,
        })
    }

    fn get(&self, p: Vec2) -> Option<NodeType> {
        if p.x >= 0 && p.x < self.size.x && p.y >= 0 && p.y < self.size.y {
            Some(self.nodes[(p.x + p.y * self.size.x) as usize].clone())
        } else {
            None
        }
    }
}

const SURROUNDING: [&str; 4] = ["R", "D", "U", "L"];

fn solve(map: &Map, initial_set: Vec<Vec2>) -> anyhow::Result<Vec<Vec2>> {
    let mut visited: HashSet<Vec2> = HashSet::new();
    let mut came_from: HashMap<Vec2, Vec2> = HashMap::new();
    let mut to_visit: VecDeque<Vec2> = VecDeque::new();

    let mut i = initial_set.clone().into_iter().collect::<VecDeque<_>>();

    to_visit.append(&mut i);

    // println!("Start: {:?}", map.start.clone());
    // println!("End: {:?}", map.end.clone());
    loop {
        if let Some(visiting) = to_visit.pop_front() {
            let visiting_height = match map.get(visiting) {
                Some(NodeType::Start) => 0,
                Some(NodeType::End) => break,
                Some(NodeType::Step(h)) => h,
                None => {
                    return Err(anyhow::anyhow!("Visiting non-existent node"));
                }
            };
            // println!("Visiting: {:?} @ {}", visiting, visiting_height);
            visited.insert(visiting);
            let mut will_visit = SURROUNDING
                .iter()
                .map(|dir| visiting.add(&Vec2::from(*dir)))
                .filter(|p| {
                    !visited.contains(p)
                        && !to_visit.contains(p)
                        && match map.get(*p) {
                            Some(NodeType::Start) => false,
                            Some(NodeType::End) => visiting_height >= 24,
                            Some(NodeType::Step(height)) => {
                                height == visiting_height
                                    || height == visiting_height + 1
                                    || height < visiting_height
                            }
                            None => false,
                        }
                })
                .collect::<VecDeque<_>>();
            // println!("Will visit: {:?}", will_visit);

            for p in will_visit.clone() {
                came_from.insert(p, visiting);
            }

            to_visit.append(&mut will_visit);

            // println!("Left to visit: {:?}", to_visit);
        } else {
            // println!("Visited: {:?}", visited);
            return Err(anyhow::anyhow!("Didn't reach the end"));
        }
    }

    let mut path: Vec<Vec2> = Vec::new();
    let mut p = map.end;
    loop {
        if initial_set.contains(&p) {
            break;
        } else {
            println!("{}", p);
            path.push(p);
            p = came_from[&p];
        }
    }

    Ok(path)
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input")?;

    let lines = input.lines();
    let map = Map::parse(lines)?;

    let mut initial_set = vec![map.start];

    for x in 0..map.size.x {
        for y in 0..map.size.y {
            let p = Vec2::from((x, y));
            match map.get(Vec2::from((x, y))) {
                Some(NodeType::Step(h)) => {
                    if h == 1 {
                        initial_set.push(p);
                    }
                }
                _ => {}
            };
        }
    }

    println!("{}", solve(&map, initial_set)?.len());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test() -> anyhow::Result<()> {
        let map = Map::parse(TEST_DATA.lines())?;
        let initial_set = vec![map.start];
        println!("{}", solve(&map, initial_set)?.len());
        Ok(())
    }
}
