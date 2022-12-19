use core::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct Vec2 {
    pub x: isize,
    pub y: isize,
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

impl From<&str> for Vec2 {
    fn from(s: &str) -> Self {
        let (x, y) = s.split_once(',').unwrap();
        Vec2 {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

impl Vec2 {
    pub fn new(x: isize, y: isize) -> Self {
        Vec2 { x, y }
    }

    pub fn origin() -> Self {
        Vec2::from((0, 0))
    }

    pub fn direction_to(&self, v: &Vec2) -> Vec2 {
        Vec2::new((v.x - self.x).signum(), (v.y - self.y).signum())
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

    pub fn manhattan(&self, v: &Vec2) -> usize {
        let delta = self.sub(v).abs();
        (delta.x + delta.y) as usize
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))?;
        Ok(())
    }
}
