use anyhow::anyhow;
use core::fmt;
use std::{collections::BTreeMap, fmt::Write, fs::read_to_string};

type Column = Vec<char>;

#[derive(Debug)]
struct Board {
    columns: BTreeMap<usize, Column>,
}

impl Board {
    fn new() -> Self {
        Board {
            columns: BTreeMap::new(),
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut keys = self.columns.keys().collect::<Vec<_>>();
        keys.sort();
        for k in keys {
            f.write_fmt(format_args!("{} : {:?}", k, self.columns[k]))?;
            f.write_char('\n')?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Move {
    how_many: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn new(how_many: usize, from: usize, to: usize) -> Self {
        Move { how_many, from, to }
    }
}

fn parse_move(line: &str) -> anyhow::Result<Move> {
    let split = line
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect::<Vec<usize>>();
    if split.len() != 3 {
        Err(anyhow!("Could not parse move"))
    } else {
        Ok(Move::new(split[0], split[1], split[2]))
    }
}

fn parse_board(lines: Vec<&str>) -> anyhow::Result<Board> {
    let mut board_lines = lines.iter().rev();

    // Last line should be the column labels, so get the number of columns from that
    let column_labels_line = match board_lines.next() {
        Some(s) => s,
        None => return Err(anyhow!("Could not find column labels")),
    };

    let column_labels = column_labels_line
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_numeric())
        .map(|(e, c)| (e, (c as u8 - b'0') as usize))
        .collect::<Vec<_>>();

    let mut board = Board::new();

    for line in board_lines {
        let chars = line.chars().collect::<Vec<char>>();
        for (idx, name) in &column_labels {
            if !chars[*idx].is_whitespace() {
                board.columns.entry(*name).or_default().push(chars[*idx]);
            }
        }
    }
    Ok(board)
}

fn play_move(board: &mut Board, played: &Move) -> anyhow::Result<()> {
    let columns = &mut board.columns;

    if let Some(from_column) = columns.get(&played.from) {
        if from_column.len() < played.how_many {
            return Err(anyhow!("Invalid move"));
        }
    } else {
        return Err(anyhow!("Invalid column"));
    }

    for _ in 0..played.how_many {
        let popped = columns.get_mut(&played.from).unwrap().pop().unwrap();
        columns.get_mut(&played.to).unwrap().push(popped);
    }

    Ok(())
}

fn play_move_2(board: &mut Board, played: &Move) -> anyhow::Result<()> {
    let columns = &mut board.columns;

    if let Some(from_column) = columns.get(&played.from) {
        if from_column.len() < played.how_many {
            return Err(anyhow!("Invalid move"));
        }
    } else {
        return Err(anyhow!("Invalid column"));
    }

    let mut temp = vec![];
    for _ in 0..played.how_many {
        let popped = columns.get_mut(&played.from).unwrap().pop().unwrap();
        temp.push(popped);
    }

    for _ in 0..played.how_many {
        let popped = temp.pop().unwrap();
        columns.get_mut(&played.to).unwrap().push(popped)
    }

    Ok(())
}

fn main() -> std::result::Result<(), anyhow::Error> {
    let input = read_to_string("input")?;

    let mut lines = input.lines();

    let board_lines = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .collect::<Vec<_>>();
    let mut board = parse_board(board_lines)?;

    for line in lines {
        let m = parse_move(line)?;
        play_move_2(&mut board, &m)?;
    }

    println!("{}", board);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let mut lines = input.lines();

        let board_lines = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .collect::<Vec<_>>();
        let mut board = parse_board(board_lines)?;
        assert_eq!(board.columns.len(), 3);
        assert_eq!(board.columns[&1], vec!['Z', 'N']);
        assert_eq!(board.columns[&2], vec!['M', 'C', 'D']);
        assert_eq!(board.columns[&3], vec!['P']);

        for line in lines {
            let m = parse_move(line)?;
            play_move(&mut board, &m)?;
        }
        assert_eq!(board.columns[&1], vec!['C']);
        assert_eq!(board.columns[&2], vec!['M']);
        assert_eq!(board.columns[&3], vec!['P', 'D', 'N', 'Z']);

        Ok(())
    }

    #[test]
    fn test2() -> anyhow::Result<()> {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let mut lines = input.lines();

        let board_lines = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .collect::<Vec<_>>();
        let mut board = parse_board(board_lines)?;
        assert_eq!(board.columns.len(), 3);
        assert_eq!(board.columns[&1], vec!['Z', 'N']);
        assert_eq!(board.columns[&2], vec!['M', 'C', 'D']);
        assert_eq!(board.columns[&3], vec!['P']);

        for line in lines {
            let m = parse_move(line)?;
            play_move_2(&mut board, &m)?;
            println!("{}", board);
        }
        assert_eq!(board.columns[&1], vec!['M']);
        assert_eq!(board.columns[&2], vec!['C']);
        assert_eq!(board.columns[&3], vec!['P', 'Z', 'N', 'D']);

        Ok(())
    }
}
