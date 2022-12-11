use std::fs;
mod monkey;

fn parse_monkeys<'a, I>(lines: I) -> anyhow::Result<Vec<monkey::Monkey>>
where I: Iterator<Item = &'a str> {
    let mut monkeys = Vec::new();
    let mut iter = lines.peekable();
    loop {
        let line = iter.peek();
        if let Some(line_str) = line {
        if (*line_str).trim().starts_with("Monkey") {
            let test_lines = iter.by_ref().take(6).collect::<Vec<_>>();
            monkeys.push(monkey::Monkey::parse(&test_lines)?);
        } else {
            iter.next();
        } }
        else { break; }
    }
    Ok(monkeys)
}

fn run_operation(op: &monkey::Operation, old: i64) -> i64 {
    match op {
        monkey::Operation::Add(x) => old + x,
        monkey::Operation::Mul(x) => old * x,
        monkey::Operation::Sqr => old * old
    }
}

fn run_monkeys(monkeys: &mut Vec<monkey::Monkey>, how_worried: i64) -> anyhow::Result<Vec<usize>> {
    let mut inspected = Vec::new();
    let all_divisible_by = monkeys.into_iter()
        .map(|m| m.test.divisible_by)
        .reduce(|acc, i| acc * i).unwrap();

    for monkey_index in 0..monkeys.len() {
        println!("Monkey {}", monkey_index);
        let mut monkey = &mut monkeys[monkey_index];

        let v = monkey.items.clone().into_iter().map( |item| {
            let mut worry_level = run_operation(&monkey.operation, item) / how_worried;
            let throw_to = 
            if worry_level % monkey.test.divisible_by == 0 {
                monkey.test.pass_true
            } else {
                monkey.test.pass_false
            };
            (throw_to, worry_level % all_divisible_by)
        }).collect::<Vec<_>>();

        monkey.items.clear();
        inspected.push(v.len());
        for (throw_to, worry_level) in v {
            monkeys[throw_to].items.push(worry_level);
        }
    }

    Ok(inspected)
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input")?;

    let lines = input.lines();
    let mut monkeys = parse_monkeys(lines)?;

    let mut total = vec![0; monkeys.len()];
    for rounds in 0..20 {
        let passes = run_monkeys(&mut monkeys, 1)?;
        total = passes.into_iter().enumerate().map(|(i,p)| total[i] + p).collect();
    }
    let mut monkey_business = total.clone();
    monkey_business.sort_by(|a,b| b.cmp(a));
    println!("{:?}", monkey_business[0] * monkey_business[1]);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{monkey::*, parse_monkeys, run_monkeys};

    #[test]
    fn test_parse_operation() -> anyhow::Result<()>
    {
        for line in TEST_DATA.lines() {
            if line.starts_with("Operation:") {
                println!("{:?}",Operation::parse(line)?);
            }
        }
        Ok(())
    }

    #[test]
    fn test_parse_test() -> anyhow::Result<()>
    {
        let it = TEST_DATA.lines();
        let mut iter = it.peekable();
        loop {
            let line = iter.peek();
            if let Some(line_str) = line {
            if line_str.trim().starts_with("Test") {
                let test_lines = iter.by_ref().take(3).collect::<Vec<_>>();
                println!("{:?}",Test::parse(&test_lines)?);
            } else {
                iter.next();
            } }
            else { break; }
        }

        Ok(())
    }

    #[test]
    fn test_parse_monkeys() -> anyhow::Result<()>{
        let it = TEST_DATA.lines();
        let mut monkeys = parse_monkeys(it)?;

        let mut total = vec![0; monkeys.len()];
        for rounds in 0..10000 {
            let passes = run_monkeys(&mut monkeys, 1)?;
            total = passes.into_iter().enumerate().map(|(i,p)| total[i] + p).collect();
        }
        let mut monkey_business = total.clone();
        monkey_business.sort_by(|a,b| b.cmp(a));

        println!("{:?}, {}", total, monkey_business[0] * monkey_business[1]);
        Ok(())
    }

    #[test]
    fn test() {
        assert!(true);
    }

    const TEST_DATA: &str = 
"Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
        If true: throw to monkey 2
        If false: throw to monkey 3

Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
        If true: throw to monkey 2
        If false: throw to monkey 0

Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
        If true: throw to monkey 1
        If false: throw to monkey 3

Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
        If true: throw to monkey 0
        If false: throw to monkey 1";
}
