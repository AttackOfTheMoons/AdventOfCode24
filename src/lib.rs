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
    fn try_translate(&self, (pt_x, pt_y): (usize, usize)) -> Option<(usize, usize)> {
        let (dir_x, dir_y) = self.coords();
        if pt_x == 0 && dir_x < 0 || pt_y == 0 && dir_y < 0 {
            return None;
        }
        Some((
            (pt_x as i32 + dir_x) as usize,
            (pt_y as i32 + dir_y) as usize,
        ))
    }
    fn translate(&self, (pt_x, pt_y): (usize, usize)) -> (usize, usize) {
        let (dir_x, dir_y) = self.coords();
        if pt_x == 0 && dir_x < 0 || pt_y == 0 && dir_y < 0 {
            panic!("Tried to move overflow direction ({self:?}): ({pt_x}, {pt_y})");
        }
        (
            (pt_x as i32 + dir_x) as usize,
            (pt_y as i32 + dir_y) as usize,
        )
    }
}

/// Find's `guard_symbol` which must exist 1 time in `map_vec`
fn find_single_instance_of_char(map_vec: &Vec<Vec<char>>, guard_symbol: char) -> (usize, usize) {
    let target_pos = map_vec
        .iter()
        .enumerate()
        .map(|(index, line)| (index, line.iter().position(|c| *c == guard_symbol)))
        .filter(|(_outer_index, inner_result)| inner_result.is_some())
        .next();

    match target_pos {
        Some((outer_index, inner_result)) => match inner_result {
            Some(inner_index) => {
                return (inner_index, outer_index);
            }
            None => panic!("Couldn't find start_x"),
        },
        None => panic!("Couldn't find start_y"),
    }
}
