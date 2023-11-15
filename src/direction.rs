#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Up,
            Self::Right => Self::Right,
            Self::Down => Self::Down,
            Self::Left => Self::Left,
        }
    }
}
