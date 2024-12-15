use std::{collections::HashSet, fs};

use log::{info, trace};

use crate::Direction;

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\input\\week_2\\day10.txt";

pub fn day10() {
    let file_contents =
        fs::read_to_string(INPUT_FILE).expect(format!("Could not read file {INPUT_FILE}").as_str());

    let topographic_vec = file_contents
        .trim()
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap_or(std::u32::MAX))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum: usize = 0;
    for (i, line) in topographic_vec.iter().enumerate() {
        for (j, &x) in line.iter().enumerate() {
            if x == 0 {
                sum += climb(&topographic_vec, (i, j));
            }
        }
    }
    info!("Sum: {sum}");
}

#[allow(dead_code)]
fn climb_pt1(vec: &Vec<Vec<u32>>, start: (usize, usize)) -> usize {
    let inner = match vec.first() {
        Some(v) => v.len(),
        None => 0,
    };
    let dim = (vec.len(), inner);

    // This is pt 1
    HashSet::<_>::from_iter(
        climb_sub(vec, dim, start, Direction::Up)
            .chain(climb_sub(vec, dim, start, Direction::Down))
            .chain(climb_sub(vec, dim, start, Direction::Left))
            .chain(climb_sub(vec, dim, start, Direction::Right)),
    )
    .len()
}

fn climb(vec: &Vec<Vec<u32>>, start: (usize, usize)) -> usize {
    let inner = match vec.first() {
        Some(v) => v.len(),
        None => 0,
    };
    let dim = (vec.len(), inner);

    climb_sub(vec, dim, start, Direction::Up)
        .chain(climb_sub(vec, dim, start, Direction::Down))
        .chain(climb_sub(vec, dim, start, Direction::Left))
        .chain(climb_sub(vec, dim, start, Direction::Right))
        .count()
}

/// This method assumes (start_x, start_y) are valid steps.
fn climb_sub(
    vec: &Vec<Vec<u32>>,
    (vec_y, vec_x): (usize, usize),
    (start_y, start_x): (usize, usize),
    dir: Direction,
) -> Box<dyn '_ + Iterator<Item = (usize, usize)>> {
    let &num = vec.get(start_y).unwrap().get(start_x).unwrap();
    trace!("stepping from ({start_x}, {start_y}) {num} {dir:?}");
    let (dir_x, dir_y) = dir.coords();
    let (result_x, result_y) = ((start_x as i32 + dir_x), (start_y as i32 + dir_y));
    if result_x < 0 || result_x as usize >= vec_x || result_y < 0 || result_y as usize >= vec_y {
        return Box::new(std::iter::empty());
    }
    let (result_x, result_y) = ((result_x) as usize, (result_y) as usize);
    let &result = vec.get(result_y).unwrap().get(result_x).unwrap();
    if num + 1 != result {
        return Box::new(std::iter::empty());
    }
    if result == 9 {
        return Box::new(std::iter::once((start_y, start_x)));
    }

    Box::new(
        climb_sub(vec, (vec_y, vec_x), (result_y, result_x), Direction::Up)
            .chain(climb_sub(
                vec,
                (vec_y, vec_x),
                (result_y, result_x),
                Direction::Down,
            ))
            .chain(climb_sub(
                vec,
                (vec_y, vec_x),
                (result_y, result_x),
                Direction::Left,
            ))
            .chain(climb_sub(
                vec,
                (vec_y, vec_x),
                (result_y, result_x),
                Direction::Right,
            )),
    )
}
