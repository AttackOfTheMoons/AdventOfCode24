use log::{debug, info, trace};

use crate::Direction;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\input\\week_2\\day12.txt";

#[derive(Debug)]
pub struct Shape {
    points: HashSet<(usize, usize)>,
    letter: char,
}

impl Shape {
    fn new(letter: char, point: (usize, usize)) -> Self {
        Shape {
            letter,
            points: HashSet::from([point]),
        }
    }

    pub fn new_with_points(letter: char, points: HashSet<(usize, usize)>) -> Self {
        Shape { letter, points }
    }

    fn perimeter(&self) -> usize {
        let mut perimeter = self.points.len() * 4;
        for &(pt_x, pt_y) in self.points.iter() {
            for dir in vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                let (dir_x, dir_y) = dir.coords();
                if pt_x == 0 && dir_x < 0 || pt_y == 0 && dir_y < 0 {
                    // Invalid move.
                    continue;
                }
                let result = (
                    (pt_x as i32 + dir_x) as usize,
                    (pt_y as i32 + dir_y) as usize,
                );
                if self.points.contains(&result) {
                    perimeter -= 1;
                }
            }
        }
        return perimeter;
    }

    pub fn sides(&self) -> usize {
        // This will be a map of (sides: is that side uniquely accounted for).
        let mut side_values = HashMap::new();
        // This is a set of all points which all of their valid sides have been added already.
        let mut visited = HashSet::new();
        let mut sides = 0;
        for &point in self.points.iter() {
            trace!("Visiting {point:?}");
            visited.insert(point);
            // Now, check out the neighbors.
            for dir in vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                let result = dir.try_translate(point);
                if result.is_none() {
                    side_values.insert((point, dir), true);
                    sides += 1;
                    continue;
                }
                let result = result.unwrap();
                if !self.points.contains(&result) {
                    side_values.insert((point, dir), true);
                    sides += 1;
                }
            }
            // All of our sides have been made, now we look at neighbors to see if they have those sides
            for dir in vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                let result = dir.try_translate(point);
                if result.is_none() {
                    continue;
                }
                let result = result.unwrap();
                // No neighbor in that direction.
                if !self.points.contains(&result) {
                    continue;
                }
                for neighbor_side_dir in vec![
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ]
                .into_iter()
                {
                    if neighbor_side_dir == dir || neighbor_side_dir.opposite() == dir {
                        continue;
                    }
                    let our_version_of_this_side_exists =
                        side_values.contains_key(&(point, neighbor_side_dir));
                    if let Some(neighbor_value) = side_values.get_mut(&(result, neighbor_side_dir))
                    {
                        let neighbors = *neighbor_value;
                        *neighbor_value = *neighbor_value && !our_version_of_this_side_exists;
                        if let Some(value) = side_values.get_mut(&(point, neighbor_side_dir)) {
                            if neighbors || our_version_of_this_side_exists {
                                sides -= 1;
                            }
                            *value = false;
                        }
                    }
                }
            }
            trace!("Saved sides: {side_values:?}");
            trace!("side count: {sides}");
        }
        debug!("{self:?} has {sides} sides");
        return sides;
    }

    fn area(&self) -> usize {
        self.points.len()
    }

    #[allow(dead_code)]
    fn pt1_price(&self) -> usize {
        self.area() * self.perimeter()
    }

    fn pt2_price(&self) -> usize {
        self.area() * self.sides()
    }

    fn merge(self, other: Shape) -> Shape {
        if self.letter != other.letter {
            panic!("Could not merge {} with {}", self.letter, other.letter);
        }

        let mut new_pts = self.points;
        new_pts.extend(other.points);
        Self::new_with_points(self.letter, new_pts)
    }

    fn touches(&self, (pt_x, pt_y): (usize, usize)) -> bool {
        for dir in vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            let (dir_x, dir_y) = dir.coords();
            if pt_x == 0 && dir_x < 0 || pt_y == 0 && dir_y < 0 {
                // Invalid move.
                continue;
            }
            let result = (
                (pt_x as i32 + dir_x) as usize,
                (pt_y as i32 + dir_y) as usize,
            );
            if self.points.contains(&result) {
                return true;
            }
        }
        return false;
    }
}

pub fn day12() {
    let file_contents =
        fs::read_to_string(INPUT_FILE).expect(format!("Could not read file {INPUT_FILE}").as_str());

    let mut shapes: Vec<Shape> = Vec::new();
    for (line_idx, line) in file_contents
        .split("\n")
        .filter(|line| !line.is_empty())
        .enumerate()
    {
        for (ch_idx, ch) in line.trim().chars().enumerate() {
            let mut new_shapes = Vec::new();
            let mut shape_result = Shape::new(ch, (ch_idx, line_idx));
            for shape in shapes {
                if shape.letter == ch && shape.touches((ch_idx, line_idx)) {
                    shape_result = shape_result.merge(shape);
                } else {
                    new_shapes.push(shape);
                }
            }
            new_shapes.push(shape_result);
            shapes = new_shapes;
        }
    }
    let mut sum = 0;
    for shape in shapes {
        sum += shape.pt2_price()
    }
    info!("Sum: {sum}")
}
