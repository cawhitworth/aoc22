use std::ops::Index;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    Add(i64),
    Mul(i64),
    Sqr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Test {
    pub divisible_by: i64,
    pub pass_true: usize,
    pub pass_false: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Monkey {
    pub id: i64,
    pub items: Vec<i64>,
    pub operation: Operation,
    pub test: Test,
}

impl Operation {
    pub fn parse(line: &str) -> anyhow::Result<Self> {
        if let Some((oper, oper_str)) = line.split_once(" = ") {
            if !oper.trim().starts_with("Operation:") {
                return Err(anyhow::anyhow!("Error parsing {} - not an operation", line));
            }

            let oper_split = oper_str.split_whitespace().collect::<Vec<_>>();
            if oper_split[0] != "old" {
                return Err(anyhow::anyhow!(
                    "Error parsing {} - function not in terms of old",
                    line
                ));
            }

            match oper_split[1] {
                "+" => {
                    if oper_split[2] == "old" {
                        return Ok(Operation::Add(2));
                    }
                    let add = oper_split[2].parse()?;
                    Ok(Operation::Add(add))
                }
                "*" => {
                    if oper_split[2] == "old" {
                        return Ok(Operation::Sqr);
                    }
                    let mul = oper_split[2].parse()?;
                    Ok(Operation::Mul(mul))
                }
                _ => Err(anyhow::anyhow!("Error parsing {} - unknown operator", line)),
            }
        } else {
            Err(anyhow::anyhow!("Error parsing {}", line))
        }
    }
}

impl Test {
    pub fn parse(lines: &[&str]) -> anyhow::Result<Self> {
        if lines.len() != 3 {
            return Err(anyhow::anyhow!(
                "Error parsing {:?} - expected three lines, got {}",
                lines,
                lines.len()
            ));
        }

        let mut t = Test {
            divisible_by: 0,
            pass_true: 0,
            pass_false: 0,
        };

        if let Some((_, divisible)) = lines[0].split_once(" by ") {
            t.divisible_by = divisible.parse()?;
        } else {
            return Err(anyhow::anyhow!("Error parsing {:?}", lines[0]));
        }

        if let Some((cond, throw_to)) = lines[1].split_once(" monkey ") {
            if cond.contains("true:") {
                t.pass_true = throw_to.parse()?;
            } else {
                return Err(anyhow::anyhow!("Error parsing {:?}", lines[1]));
            }
        } else {
            return Err(anyhow::anyhow!("Error parsing {:?}", lines[1]));
        }

        if let Some((cond, throw_to)) = lines[2].split_once(" monkey ") {
            if cond.contains("false:") {
                t.pass_false = throw_to.parse()?;
            } else {
                return Err(anyhow::anyhow!("Error parsing {:?}", lines[2]));
            }
        } else {
            return Err(anyhow::anyhow!("Error parsing {:?}", lines[2]));
        }

        Ok(t)
    }
}

impl Monkey {
    pub fn parse(lines: &[&str]) -> anyhow::Result<Monkey> {
        let id: i64 = {
            if !lines[0].trim().starts_with("Monkey") {
                return Err(anyhow::anyhow!("Error parsing {} - no monkey", lines[0]));
            }
            if let Some((_, id)) = lines[0].split_once(" ") {
                if let Some(id_num) = id.strip_suffix(":") {
                    id_num.parse()?
                } else {
                    return Err(anyhow::anyhow!("Error parsing {}", lines[0]));
                }
            } else {
                return Err(anyhow::anyhow!("Error parsing {}", lines[0]));
            }
        };

        let items: Vec<i64> = {
            if !lines[1].trim().starts_with("Starting items") {
                return Err(anyhow::anyhow!("Error parsing {} - no items", lines[1]));
            }
            if let Some((_, items)) = lines[1].split_once(":") {
                items
                    .split(",")
                    .map(|i| i.trim().parse().unwrap())
                    .collect::<Vec<_>>()
            } else {
                return Err(anyhow::anyhow!("Error parsing {}", lines[0]));
            }
        };

        let operation = Operation::parse(lines[2])?;

        let test = Test::parse(&lines[3..6])?;

        Ok(Monkey {
            id,
            items,
            operation,
            test,
        })
    }
}
