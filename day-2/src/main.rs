use std::{collections::HashMap, fs::File, error::Error, io::{BufReader, BufRead}};

#[derive(PartialEq, Debug, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn will_beat(&self, other: &Self) -> bool {
        self.beats() == *other
    }

    fn beats(&self) -> Self {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper
        }
    }

    fn beaten_by(&self) -> Self {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock
        }
    }
}

#[derive(Debug, PartialEq)]
enum Player {
    Player(u32),
    None,
}

fn winner(player_1: &Move, player_2: &Move) -> Player {
    if player_1.will_beat(&player_2) {
        Player::Player(1)
    } else if player_2.will_beat(&player_1) {
        Player::Player(2)
    } else {
        Player::None
    }
}

fn moves_for_line(line: &str) -> (Move, Move) {

    let s: Vec<&str> = line.split(' ').collect();

    let player_1_move = match s[0] {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        _ => panic!("Invalid move")
    };
    
    let player_2_move = match s[1] {
        "X" => Move::Rock,
        "Y" => Move::Paper,
        "Z" => Move::Scissors,
        _ => panic!("Invalid move")
    };

    (player_1_move, player_2_move)
}

fn move_and_result_for_line(line: &str) -> (Move, Player) {
    let s: Vec<&str> = line.split(' ').collect();

    let player_1_move = match s[0] {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        _ => panic!("Invalid move")
    };
    
    let result = match s[1] {
        "X" => Player::Player(1),
        "Y" => Player::None,
        "Z" => Player::Player(2),
        _ => panic!("Invalid move")
    };

    (player_1_move, result)
}

fn move_to_play(player_1_move: &Move, winner: &Player) -> Move {
    match winner {
        Player::Player(1) => player_1_move.beats(),
        Player::Player(2) => player_1_move.beaten_by(),
        Player::None => *player_1_move,
        Player::Player(_) => panic!("Only two players supported")
    }
}

fn score_for_game(them: &Move, me: &Move) -> u32 {
    let winner = winner(&them, &me);
    let mut score = 
    match winner {
        Player::Player(1) => 0,
        Player::Player(2) => 6,
        Player::Player(_) => panic!("Only two players supported"),
        Player::None => 3,
    };
    score += match me {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };

    score
}

fn main() -> Result<(), Box<dyn Error>>{
    let mut score_1 = 0;
    let mut score_2 = 0;
    let input = File::open("input")?;
    {
        let reader = BufReader::new(input);
        for line in reader.lines() {

            let line_str = line?;
            let (them, me) = moves_for_line(&line_str);
            score_1 += score_for_game(&them, &me);

            let (them, result) = move_and_result_for_line(&line_str);
            score_2 += score_for_game(&them, &move_to_play(&them, &result))

        }
    }

    println!("{}", score_1);
    println!("{}", score_2);

    Ok(())
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn check_rules() {
        assert_eq!(winner(&Move::Rock, &Move::Scissors), Player::Player(1));
        assert_eq!(winner(&Move::Scissors, &Move::Scissors), Player::None);
        assert_eq!(winner(&Move::Scissors, &Move::Rock), Player::Player(2));
        assert_eq!(winner(&Move::Scissors, &Move::Paper), Player::Player(1));
        assert_eq!(winner(&Move::Paper, &Move::Rock), Player::Player(1));
    }

    #[test]
    fn check_moves_for_line() {
        assert_eq!(moves_for_line("A X"), (Move::Rock, Move::Rock));
        assert_eq!(moves_for_line("B Y"), (Move::Paper, Move::Paper));
        assert_eq!(moves_for_line("C Z"), (Move::Scissors, Move::Scissors));
    }

    #[test]
    fn check_scores_for_game1() {
        assert_eq!(score_for_game(&Move::Rock, &Move::Scissors), 3);
        assert_eq!(score_for_game(&Move::Rock, &Move::Paper), 8);
        assert_eq!(score_for_game(&Move::Rock, &Move::Rock), 4);
    }

    #[test]
    fn check_test_data_rule1() {
        let test_data = vec![
            ("A Y", 8),
            ("B X", 1),
            ("C Z", 6)
        ];

        for (line, score) in test_data {
            let (them, me) = moves_for_line(line);
            assert_eq!(score_for_game(&them, &me), score);
        }
    }

    #[test]
    fn check_move_and_result_for_line()
    {
        assert_eq!(move_and_result_for_line("A X"), (Move::Rock, Player::Player(1)));
        assert_eq!(move_and_result_for_line("B Y"), (Move::Paper, Player::None));
        assert_eq!(move_and_result_for_line("C Z"), (Move::Scissors, Player::Player(2)));
    }

    #[test]
    fn check_move_to_play()
    {
        assert_eq!(move_to_play(&Move::Rock, &Player::Player(1)), Move::Scissors);
        assert_eq!(move_to_play(&Move::Rock, &Player::Player(2)), Move::Paper);
        assert_eq!(move_to_play(&Move::Rock, &Player::None), Move::Rock);
    
    }
    #[test]
    fn check_test_data_rule2() {
        let test_data = vec![
            ("A Y", 4),
            ("B X", 1),
            ("C Z", 7),
            ("B Z", 9)
        ];

        for (line, score) in test_data {
            let (them, winner) = move_and_result_for_line(line);
            assert_eq!(score_for_game(&them, &move_to_play(&them, &winner)), score);
        }
    }
}