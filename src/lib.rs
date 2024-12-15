pub mod week1;
pub mod week2;
pub mod week3;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Coord2D {
    x: i64,
    y: i64,
}

impl Coord2D {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Direction {
    fn coords(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
    fn translate(&self, (pt_x, pt_y): (usize, usize)) -> Option<(usize, usize)> {
        let (dir_x, dir_y) = self.coords();
        if pt_x == 0 && dir_x < 0 || pt_y == 0 && dir_y < 0 {
            return None;
        }
        Some((
            (pt_x as i32 + dir_x) as usize,
            (pt_y as i32 + dir_y) as usize,
        ))
    }
}
