use std::{cmp::Ordering, collections::binary_heap::Iter, fmt, fs};

struct Arena<T> {
    items: Vec<T>,
}

#[derive(Copy, Clone)]
struct Index {
    i: usize,
}

impl<T> Arena<T> {
    fn new() -> Self {
        Arena { items: Vec::new() }
    }

    fn get(&self, id: Index) -> &T {
        &self.items[id.i]
    }

    fn get_mut(&mut self, id: Index) -> &mut T {
        &mut self.items[id.i]
    }

    fn insert(&mut self, item: T) -> Index {
        self.items.push(item);
        Index {
            i: self.items.len() - 1,
        }
    }
}

#[derive(Clone)]
enum Node {
    Int(i32),
    List(Vec<Index>),
}

fn append_to(idx: Index, list_idx: Index, arena: &mut Arena<Node>) -> anyhow::Result<()> {
    let node = arena.get_mut(list_idx);

    match node {
        Node::List(l) => {
            l.push(idx);
        }
        _ => {
            return Err(anyhow::anyhow!("Cannot append to non-list node"));
        }
    }
    Ok(())
}

fn get_number(chars: &Vec<char>, idx: usize) -> anyhow::Result<(i32, usize)> {
    let mut next = idx;
    let mut str = "".to_string();
    loop {
        if chars[next].is_digit(10) {
            str += &chars[next].to_string();
            next += 1;
        } else {
            break;
        }
    }
    return Ok((str.parse()?, next));
}

fn parse(line: &str, arena: &mut Arena<Node>) -> anyhow::Result<Index> {
    let chars = line.chars().collect::<Vec<_>>();
    let mut chars_idx = 0;
    let mut list_stack: Vec<Index> = Vec::new();
    let mut root_index = None;
    loop {
        let c = chars[chars_idx];
        match c {
            '[' => {
                chars_idx += 1;
                let list = Vec::new();
                let new_idx = arena.insert(Node::List(list));

                if let Some(list_idx) = list_stack.last() {
                    append_to(new_idx, *list_idx, arena)?;
                }

                list_stack.push(new_idx);
                if root_index.is_none() {
                    root_index = Some(new_idx);
                }
            }
            ']' => {
                chars_idx += 1;
                list_stack.pop();
            }
            i if i.is_numeric() => {
                let (d, new_char_idx) = get_number(&chars, chars_idx)?;
                let new_node = Node::Int(d);
                let new_idx = arena.insert(new_node);
                let list_idx = *list_stack.last().unwrap();
                append_to(new_idx, list_idx, arena)?;
                chars_idx = new_char_idx;
            }
            _ => {
                chars_idx += 1;
            }
        }
        if chars_idx >= chars.len() {
            break;
        }
    }

    match root_index {
        Some(i) => Ok(i),
        None => Err(anyhow::anyhow!("No root list found")),
    }
}

fn walk(arena: &Arena<Node>, idx: Index) {
    match arena.get(idx) {
        Node::Int(x) => {
            print!("{}, ", x);
        }
        Node::List(l) => {
            print!("[");
            for i in l {
                walk(arena, *i);
            }
            print!("]");
        }
    }
}

fn promote(idx: Index, arena: &mut Arena<Node>) -> Index {
    let list = vec![idx];
    arena.insert(Node::List(list))
}

fn compare(root1: Index, root2: Index, arena: &mut Arena<Node>) -> anyhow::Result<Ordering> {
    let idx_l = arena.get(root1).clone();
    let idx_r = arena.get(root2).clone();
    match (idx_l, idx_r) {
        (Node::List(lhs), Node::List(rhs)) => {
            let lhs_len = lhs.len();
            let rhs_len = rhs.len();
            let zipped = lhs.into_iter().zip(rhs.into_iter()).collect::<Vec<_>>();
            for (l, r) in zipped {
                let result = compare(l, r, arena)?;
                if result != Ordering::Equal {
                    return Ok(result);
                }
            }
            Ok(lhs_len.cmp(&rhs_len))
        }

        (Node::Int(_), Node::List(_)) => {
            let promoted = promote(root1, arena);
            return compare(promoted, root2, arena);
        }

        (Node::List(_), Node::Int(_)) => {
            let promoted = promote(root2, arena);
            return compare(root1, promoted, arena);
        }

        (Node::Int(lhs), Node::Int(rhs)) => Ok(lhs.cmp(&rhs)),

        (_, _) => {
            return Err(anyhow::anyhow!("Not implemented yet"));
        }
    }
}

fn compare_lines(lhs: &str, rhs: &str) -> anyhow::Result<Ordering> {
    let mut arena = Arena::new();
    let root1 = parse(lhs, &mut arena)?;
    let root2 = parse(rhs, &mut arena)?;
    let result = compare(root1, root2, &mut arena)?;
    Ok(result)
}

fn score<'a, I>(lines: &mut I) -> anyhow::Result<usize>
where
    I: Iterator<Item = &'a str>,
{
    let mut pair = 0;
    let mut score = 0;
    loop {
        if let Some(line1) = lines.next() {
            pair += 1;
            let line2 = lines.next().unwrap();
            lines.next();
            let result = compare_lines(line1, line2)?;
            match result {
                Ordering::Less => {
                    score += pair;
                }
                Ordering::Equal => {
                    return Err(anyhow::anyhow!("Non-ordered pair {} vs {}", line1, line2));
                }
                Ordering::Greater => {}
            }
        } else {
            break;
        }
    }
    Ok(score)
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input")?;
    {
        let input_clone = input.clone();
        let mut lines = input_clone.lines();
        println!("{}", score(&mut lines)?);
    }

    {
        let input_clone = input.clone();
        let mut lines = input_clone
            .lines()
            .filter(|l| !l.is_empty())
            .collect::<Vec<_>>();
        lines.push("[[2]]");
        lines.push("[[6]]");
        lines.sort_by(|lhs, rhs| compare_lines(lhs, rhs).unwrap());

        for (l, _) in lines
            .iter()
            .enumerate()
            .filter(|(i, s)| *s == &"[[2]]" || *s == &"[[6]]")
        {
            println!("{:?}", l + 1);
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_arena() {
        let mut arena: Arena<Node> = Arena::new();

        let mut l = Vec::new();
        l.push(arena.insert(Node::Int(1)));
        let mut l2 = Vec::new();
        l2.push(arena.insert(Node::Int(2)));
        l.push(arena.insert(Node::List(l2)));
        l.push(arena.insert(Node::Int(3)));
        let id = arena.insert(Node::List(l));
    }

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        let mut arena: Arena<Node> = Arena::new();
        let idx = parse("[1,[2,3,[4]],5,[6,7],8,9,10]", &mut arena)?;
        walk(&arena, idx);
        Ok(())
    }

    #[test]
    fn test_basic_compare() -> anyhow::Result<()> {
        let mut arena: Arena<Node> = Arena::new();
        let idx = parse("[1,2,3,4]", &mut arena)?;
        let idx2 = parse("[1,2,3,4,5]", &mut arena)?;
        let result = compare(idx, idx2, &mut arena)?;
        assert_eq!(result, Ordering::Less);
        Ok(())
    }

    #[test]
    fn test_empty_compare() -> anyhow::Result<()> {
        let mut arena: Arena<Node> = Arena::new();
        let idx = parse("[[[]]]", &mut arena)?;
        let idx2 = parse("[[]]", &mut arena)?;
        let result = compare(idx, idx2, &mut arena)?;
        assert_eq!(result, Ordering::Greater);
        Ok(())
    }

    #[test]
    fn test_promote_compare() -> anyhow::Result<()> {
        let mut arena: Arena<Node> = Arena::new();
        let idx = parse("[1,2,3]", &mut arena)?;
        let idx2 = parse("[1,2,[4]]", &mut arena)?;
        let result = compare(idx, idx2, &mut arena)?;
        assert_eq!(result, Ordering::Less);
        Ok(())
    }

    #[test]
    fn test_from_samples() -> anyhow::Result<()> {
        let test_cases = vec![
            // ("[1,1,3,1,1]", "[1,1,5,1,1]", Ordering::Less),
            // ("[[1],[2,3,4]]", "[[1],4]", Ordering::Less),
            ("[9]", "[[8,7,6]]", Ordering::Greater),
            // ("[[4,4],4,4]", "[[4,4],4,4,4]", Ordering::Less),
            // ("[7,7,7,7]", "[7,7,7]", Ordering::Greater),
            // ("[]", "[3]", Ordering::Less),
            // ("[[[]]]", "[[]]", Ordering::Greater),
            // (
            //     "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            //     "[1,[2,[3,[4,[5,6,0]]]],8,9]",
            //     Ordering::Greater,
            // ),
        ];

        for (lhs, rhs, expected_result) in test_cases {
            let mut arena: Arena<Node> = Arena::new();
            let idx = parse(lhs, &mut arena)?;
            let idx2 = parse(rhs, &mut arena)?;
            walk(&arena, idx);
            println!();
            walk(&arena, idx2);
            println!();
            let result = compare(idx, idx2, &mut arena)?;
            assert_eq!(
                result, expected_result,
                "{} {} {:?}",
                lhs, rhs, expected_result
            );
        }
        Ok(())
    }

    #[test]
    fn test_score() -> anyhow::Result<()> {
        let test_data = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        let mut lines = test_data.lines();
        assert_eq!(score(&mut lines)?, 13);
        Ok(())
    }

    #[test]
    fn failing_test() -> anyhow::Result<()> {
        let line1 = "[[[[1,8,3,7],9,7,[],[6,3,5,2,9]],3,4,[],[9]]]";
        let line2 = "[[[[1],8,6,[]],9],[[7,[2,8,0,9]],[[4,2,5,5],5],0],[3,[[3,1,8],10,[],0,5],6,[]],[[9,[2,8,0,0,1],[],[1,1,8]],[9,9,[2,9,1,1,1],4,2],[[1],8,[0,5,6,7,8]],[7,7,[4,6,10,10],[4,0,9]],3]]";
        let mut arena: Arena<Node> = Arena::new();
        let idx = parse(line1, &mut arena)?;
        walk(&arena, idx);
        println!("");
        let idx2 = parse(line2, &mut arena)?;
        let result = compare(idx, idx2, &mut arena)?;
        assert_eq!(result, Ordering::Greater);
        Ok(())
    }

    #[test]
    fn test_sort() {
        let test_data = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        let mut lines = test_data
            .lines()
            .filter(|l| !l.is_empty())
            .collect::<Vec<_>>();
        lines.push("[[2]]");
        lines.push("[[6]]");
        lines.sort_by(|lhs, rhs| compare_lines(lhs, rhs).unwrap());

        for l in lines
            .iter()
            .enumerate()
            .filter(|(i, s)| *s == &"[[2]]" || *s == &"[[6]]")
        {
            println!("{:?}", l);
        }
    }
}
