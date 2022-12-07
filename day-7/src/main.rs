use anyhow::{anyhow, Error};

use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

#[derive(PartialEq, Debug)]
enum ParsedLine {
    Cd(String),
    Ls,
    File(usize),
    Dir(String),
}

fn parse_line(line: &str) -> anyhow::Result<ParsedLine> {
    let split = line.split_whitespace().collect::<Vec<_>>();

    match split[0] {
        "dir" => Ok(ParsedLine::Dir(split[1].to_string())),
        "$" => match split[1] {
            "ls" => Ok(ParsedLine::Ls),
            "cd" => Ok(ParsedLine::Cd(split[2].to_string())),
            _ => Err(anyhow!("Unrecognised command")),
        },
        digits => Ok(ParsedLine::File(digits.parse()?)),
    }
}

fn walk_dirs<'a, I>(mut lines: I) -> anyhow::Result<HashMap<String, usize>>
where
    I: Iterator<Item = &'a str>,
{
    let mut dirSizes: HashMap<String, usize> = HashMap::new();
    let mut currentDir = "".to_string();
    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        let pl = parse_line(&line.unwrap())?;
        match pl {
            ParsedLine::Cd(dir) => {
                if dir == "..".to_string() {
                    let end = currentDir.rfind('/');
                    if let Some(index) = end {
                        let (new_dir, _) = currentDir.split_at(index);
                        currentDir = new_dir.to_string();
                    } else {
                        return Err(anyhow!("Cannot cd any farther"));
                    }
                } else if dir.starts_with("/") {
                        currentDir = dir
                } else {
                    currentDir = currentDir + &format!("/{}", dir);
                }
            }
            ParsedLine::Ls => {}
            ParsedLine::File(size) => {
                *dirSizes.entry(currentDir.clone()).or_default() += size;
            }
            ParsedLine::Dir(_) => {}
        }
    }
    return Ok(dirSizes);
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?;

    {
        let mut lines = input.lines();
        let dirSizes = walk_dirs(lines)?;
    let mut total_size = 0;
    for (_, size) in dirSizes {
        if size <= 100000 {
            total_size += size;
        }
    }

    println!("Score: {}", total_size);

    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const testData : &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        let mut it = testData.lines();
        let mut pl = parse_line(it.next().unwrap())?;
        assert_eq!(pl, ParsedLine::Cd("/".to_string()));
        pl = parse_line(it.next().unwrap())?;
        assert_eq!(pl, ParsedLine::Ls);
        pl = parse_line(it.next().unwrap())?;
        assert_eq!(pl, ParsedLine::Dir("a".to_string()));
        pl = parse_line(it.next().unwrap())?;
        assert_eq!(pl, ParsedLine::File(14848514));

        Ok(())
    }

    #[test]
    fn test_sum() -> anyhow::Result<()> {
        let mut it = testData.lines();
        let dirSizes = walk_dirs(it)?;
        let mut total_size = 0;
        for (dir, size) in dirSizes {
            println!("{} {}", dir, size);
            if size <= 100000 {
                total_size += size;
            }
        }
        assert_eq!(total_size, 95437);
        Ok(())
    }
}
