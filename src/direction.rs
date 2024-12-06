#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    pub fn get_index(&self, x: i64, y: i64, n: i64) -> (i64, i64) {
        match self {
            Direction::Up => (x - n, y),
            Direction::Down => (x + n, y),
            Direction::UpRight => (x - n, y + n),
            Direction::UpLeft => (x - n, y - n),
            Direction::DownRight => (x + n, y + n),
            Direction::DownLeft => (x + n, y - n),
            Direction::Right => (x, y + n),
            Direction::Left => (x, y - n),
        }
    }

    pub fn turn_right_90(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::UpRight => Direction::DownRight,
            Direction::Right => Direction::Down,
            Direction::DownRight => Direction::DownLeft,
            Direction::Down => Direction::Left,
            Direction::DownLeft => Direction::UpLeft,
            Direction::Left => Direction::Up,
            Direction::UpLeft => Direction::UpRight,
        }
    }
}
