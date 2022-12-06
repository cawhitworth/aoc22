

use std::{
    collections::{HashSet, VecDeque},
    fs::{read_to_string},
};

struct Detector {
    queue: VecDeque<char>,
    capacity: usize,
}

impl Detector {
    fn new(length: usize) -> Self {
        Detector {
            queue: VecDeque::new(),
            capacity: length,
        }
    }

    pub fn push_and_check(&mut self, c: char) -> bool {
        if self.queue.len() == self.capacity {
            self.queue.pop_front();
        }
        self.queue.push_back(c);
        let set_length = self.queue.iter().collect::<HashSet<&char>>().len();

        set_length == self.capacity
    }

    pub fn detect(&mut self, input: &str) -> Option<usize> {
        if input.len() < self.capacity {
            return None;
        }
        for (i, c) in input.chars().enumerate() {
            if self.push_and_check(c) {
                return Some(i + 1);
            }
        }
        None
    }
}

fn main() -> anyhow::Result<()> {
    let input = read_to_string("input")?;

    let mut start_detector = Detector::new(4);

    println!("{:?}", start_detector.detect(&input));

    let mut message_detector = Detector::new(14);

    println!("{:?}", message_detector.detect(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let test_data = vec![
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", Some(5)),
            ("nppdvjthqldpwncqszvftbrmjlhg", Some(6)),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", Some(10)),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", Some(11)),
            ("abcd", Some(4)),
            ("abc", None),
        ];
        for (input, expected) in test_data {
            let mut detector = Detector::new(4);
            assert_eq!(detector.detect(input), expected, "{}", input);
        }
    }
}
