use anyhow::Error;

use std::{
    fmt::Write,
    fs::{self, File},
    io::{BufRead, BufReader},
};

struct HeightMap {
    m: Vec<u8>,
    w: usize,
    h: usize,
}

impl HeightMap {
    fn from(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<_>>();

        let mut m = Vec::new();
        let width = lines[0].len();
        let height = lines.len();
        m.resize(lines[0].len() * lines.len(), 0);
        for (y, line) in lines.into_iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                m[y * width + x] = c as u8 - b'0';
            }
        }

        HeightMap {
            m,
            w: width,
            h: height,
        }
    }

    fn height(&self, x: usize, y: usize) -> u8 {
        self.m[y * self.h + x]
    }

    fn visible(&self, x: usize, y: usize) -> bool {
        if x == 0 || y == 0 || x == self.w - 1 || y == self.h - 1 {
            return true;
        }

        // From left
        let index_left = (1..=x).fold(0, |acc, i| {
            if self.height(i, y) > self.height(acc, y) {
                i
            } else {
                acc
            }
        });
        if index_left == x {
            return true;
        }
        // From right
        let index_right = (x + 1..self.w).fold(x, |acc, i| {
            if self.height(i, y) >= self.height(acc, y) {
                i
            } else {
                acc
            }
        });
        if index_right == x {
            return true;
        }

        // From top
        let index_top = (1..=y).fold(0, |acc, i| {
            if self.height(x, i) > self.height(x, acc) {
                i
            } else {
                acc
            }
        });
        if index_top == y {
            return true;
        }
        // From bottom
        let index_bottom = (y + 1..self.h).fold(y, |acc, i| {
            if self.height(x, i) >= self.height(x, acc) {
                i
            } else {
                acc
            }
        });
        if index_bottom == y {
            return true;
        }
        false
    }

    fn count_visible(&self) -> usize {
        let mut visible = 0;
        for x in 0..self.w {
            for y in 0..self.h {
                visible += self.visible(x, y) as usize;
            }
        }

        visible
    }

    fn max_height_in_range<I>(&self, iter: I) -> (usize, usize, u8)
    where
        I: Iterator<Item = (usize, usize)>,
    {
        let (x, y) = iter
            .max_by(|(x1, y1), (x2, y2)| self.height(*x1, *y1).cmp(&self.height(*x2, *y2)))
            .unwrap();
        (x, y, self.height(x, y))
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        // Highest in 0..x, y
        let h = self.height(x, y);

        let score_left = if x == 0 {
            0
        } else {
            let (rx, ry, rh) = self.max_height_in_range((0..x).map(|i| (i, y)));
            if h > rh {
                x
            } else {
                x - rx
            }
        };

        let score_right = if x == self.w - 1 {
            0
        } else {
            let (rx, ry, rh) = self.max_height_in_range((x + 1..self.w).rev().map(|i| (i, y)));
            if h > rh {
                self.w - (x + 1)
            } else {
                rx - x
            }
        };

        let score_up = if y == 0 {
            y
        } else {
            let (rx, ry, rh) = self.max_height_in_range((0..y).map(|i| (x, i)));
            if h > rh {
                y
            } else {
                y - ry
            }
        };

        let score_down = if y == self.h - 1 {
            0
        } else {
            let (rx, ry, rh) = self.max_height_in_range((y + 1..self.h).rev().map(|i| (x, i)));
            if h > rh {
                self.h - (y + 1)
            } else {
                ry - y
            }
        };
        score_left * score_right * score_up * score_down
    }
}

impl std::fmt::Display for HeightMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.h {
            for x in 0..self.w {
                f.write_fmt(format_args!("{}", self.height(x, y)))?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input")?;
    let hm = HeightMap::from(&input);
    println!("Visible: {}", hm.count_visible());

    let mut max = 0;
    for x in 0..hm.w {
        for y in 0..hm.h {
            let ss = hm.scenic_score(x, y);
            if ss > max {
                max = ss;
            }
        }
    }

    println!("Scenic score: {}", max);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test() {
        let hm = HeightMap::from(TEST_DATA);
        println!("{}", hm);
        assert_eq!(hm.count_visible(), 21);
    }

    #[test]
    fn test2() {
        let hm = HeightMap::from(TEST_DATA);
        assert_eq!(hm.scenic_score(2, 1), 4);
        assert_eq!(hm.scenic_score(2, 3), 8);
        assert_eq!(hm.scenic_score(2, 0), 0);
    }
}
