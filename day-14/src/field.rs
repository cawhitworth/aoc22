use crate::vec2::Vec2;
use itertools::Itertools;
use anyhow::{anyhow, Result};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Cell {
    Empty,
    Rock,
    Sand
}

pub struct Field {
    origin: Vec2,
    size: Vec2,
    cells: Vec<Cell>
}

impl Field {
    pub fn new(top_left: Vec2, bottom_right: Vec2) -> Self {
        let size = bottom_right.sub(&top_left).add(&Vec2::new(1,1));
        Field { origin: top_left, size, cells: vec![Cell::Empty; (size.x * size.y) as usize] }
    }

    fn cell_offset(&self, p: Vec2) -> Result<usize> {
        let v = Vec2::new(p.x - self.origin.x, p.y - self.origin.y);
        if v.x >= 0 && v.x < self.size.x && v.y >= 0 && v.y < self.size.y {
            Ok((v.x + v.y * self.size.x) as usize)
        } else {
            Err(anyhow!("Cell out of bounds"))
        }
    }

    pub fn get(&self, p: Vec2) -> Result<Cell> {
        Ok(self.cells[self.cell_offset(p)?])
    }

    pub fn put(&mut self, p: Vec2, c: Cell) -> Result<()> {
        let offset = self.cell_offset(p)?;
        self.cells[offset] = c;
        Ok(())
    }

    pub fn draw_lines(&mut self, points: Vec<Vec2>) -> Result<()> {
        for (first, second) in points.iter().tuple_windows() {
            let dir = first.direction_to(second);
            let mut pos = *first;
            loop {
                self.put(pos, Cell::Rock)?;
                if pos == *second {
                    break;
                }
                pos = pos.add(&dir);
            }
        }
        Ok(())
    }
}