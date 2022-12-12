use core::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct Vec2 {
    pub x: isize,
    pub y: isize,
}

impl From<&str> for Vec2 {
    fn from(item: &str) -> Vec2 {
        match item {
            "U" => Vec2 { x: 0, y: 1 },
            "R" => Vec2 { x: 1, y: 0 },
            "D" => Vec2 { x: 0, y: -1 },
            "L" => Vec2 { x: -1, y: 0 },
            _ => panic!("Invalid direction"),
        }
    }
}

impl From<(isize, isize)> for Vec2 {
    fn from(item: (isize, isize)) -> Vec2 {
        Vec2 {
            x: item.0,
            y: item.1,
        }
    }
}

impl From<Vec2> for (isize, isize) {
    fn from(v: Vec2) -> Self {
        (v.x, v.y)
    }
}

impl Vec2 {
    pub fn new(x: isize, y: isize) -> Self {
        Vec2 { x, y }
    }

    pub fn origin() -> Self {
        Vec2::from((0, 0))
    }

    pub fn direction(v1: &Vec2, v2: &Vec2) -> Vec2 {
        Vec2::new((v1.x - v2.x).signum(), (v1.y - v2.y).signum())
    }

    pub fn add(&self, v: &Vec2) -> Vec2 {
        Vec2::new(self.x + v.x, self.y + v.y)
    }

    pub fn sub(&self, v: &Vec2) -> Vec2 {
        Vec2::new(self.x - v.x, self.y - v.y)
    }

    pub fn abs(&self) -> Vec2 {
        Vec2::new(self.x.abs(), self.y.abs())
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))?;
        Ok(())
    }
}
