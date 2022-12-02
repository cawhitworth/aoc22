use std::collections::HashMap;

#[derive(PartialEq, Debug, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn beats(&self, other: &Self) -> bool {
        match (self, other) {
            (Move::Rock, Move::Scissors) => true,
            (Move::Paper, Move::Rock) => true,
            (Move::Scissors, Move::Paper) => true,
            (_, _) => false,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Player {
    Player(u32),
    None,
}

fn winner(player_1: &Move, player_2: &Move) -> Player {
    if player_1.beats(&player_2) {
        Player::Player(1)
    } else if player_2.beats(&player_1) {
        Player::Player(2)
    } else {
        Player::None
    }
}

fn moves_for_line(line: &str) -> (Move, Move) {
    let player_1_moves = HashMap::from ( [
        ("A", Move::Rock),
        ("B", Move::Paper),
        ("C", Move::Scissors)
    ]);

    let player_2_moves = HashMap::from ( [
        ("X", Move::Rock),
        ("Y", Move::Paper),
        ("Z", Move::Scissors)
    ]);

    let s: Vec<&str> = line.split(' ').collect();

    (player_1_moves[s[0]], player_2_moves[s[1]])
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

fn main() {
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
    fn check_line() {
        assert_eq!(moves_for_line("A X"), (Move::Rock, Move::Rock));
        assert_eq!(moves_for_line("B Y"), (Move::Paper, Move::Paper));
        assert_eq!(moves_for_line("C Z"), (Move::Scissors, Move::Scissors));
    }

    #[test]
    fn check_scores() {
        assert_eq!(score_for_game(&Move::Rock, &Move::Scissors), 3);
        assert_eq!(score_for_game(&Move::Rock, &Move::Paper), 8);
        assert_eq!(score_for_game(&Move::Rock, &Move::Rock), 4);
    }

    #[test]
    fn check_test_data() {
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
}