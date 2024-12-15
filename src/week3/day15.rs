use std::fs;

use log::{debug, info, trace};

use crate::Direction;

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\day15.txt";

pub fn day15() {
    // Parsing input.
    let file_contents =
        fs::read_to_string(INPUT_FILE).expect(format!("Could not read file {INPUT_FILE}").as_str());

    // part_1(file_contents);
    part_2(file_contents);
}

fn part_2(file_contents: String) {
    let mut parts = file_contents.split("\r\n\r\n");

    let mut map_vec = Vec::new();

    for line in parts.next().expect("Map part not found").split("\r\n") {
        let mut line_vec = Vec::new();
        for ch in line.chars() {
            match ch {
                '#' => {
                    line_vec.push('#');
                    line_vec.push('#');
                }
                'O' => {
                    line_vec.push('[');
                    line_vec.push(']');
                }
                '.' => {
                    line_vec.push('.');
                    line_vec.push('.');
                }
                '@' => {
                    line_vec.push('@');
                    line_vec.push('.');
                }
                _ => {
                    panic!("Map symbol not recognized during parsing.");
                }
            }
        }
        map_vec.push(line_vec);
    }

    let directions: Vec<Direction> = parts
        .next()
        .expect("directions not found")
        .chars()
        .filter(|&ch| ch != '\r' && ch != '\n')
        .map(|ch| match ch {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => panic!("Direction symbol not recognized during parsing"),
        })
        .collect();
    let (mut guard_x, mut guard_y) = find_guard(&map_vec);

    for (dir_idx, dir) in directions.into_iter().enumerate() {
        if can_move_pt2(guard_x, guard_y, dir, &map_vec, false) {
            move_pt2(guard_x, guard_y, dir, '.', &mut map_vec, false);
            let (dir_x, dir_y) = dir.coords();
            let (res_x, res_y) = (guard_x as i32 + dir_x, guard_y as i32 + dir_y);
            (guard_x, guard_y) = (res_x as usize, res_y as usize);
        }
        debug!("Step #{dir_idx} - Moved {dir:?}");
        for line in map_vec.iter() {
            debug!("{}", line.iter().collect::<String>());
        }
    }

    let mut sum = 0;
    for (line_idx, line) in map_vec.into_iter().enumerate() {
        for (ch_idx, ch) in line.into_iter().enumerate() {
            if ch == '[' {
                sum += 100 * line_idx + ch_idx;
            }
        }
    }
    info!("Sum of all boxes' GPS coords: {sum}");
}

fn move_pt2(
    pos_x: usize,
    pos_y: usize,
    dir: Direction,
    char_to_put: char,
    vec: &mut Vec<Vec<char>>,
    ignore_neighbor: bool,
) {
    trace!("move_pt2(pos_x: {pos_x}, pos_y: {pos_y}, dir: {dir:?}, char_to_put: {char_to_put}, vec, ignore_neighbor: {ignore_neighbor})");
    // This represents the next character.
    let current = match vec.get(pos_y) {
        Some(line) => match line.get(pos_x) {
            Some(&ch) => ch,
            None => panic!("Moving an out of bounds point.x ({pos_x}, {pos_y})"),
        },
        None => panic!("Moving an out of bounds point.y ({pos_x}, {pos_y})"),
    };
    if current == '.' {
        *vec.get_mut(pos_y).unwrap().get_mut(pos_x).unwrap() = char_to_put;
        return;
    }
    if current == '#' {
        panic!("Tried to move a wall");
    }
    // Moving Guard or a Box
    let (dir_x, dir_y) = dir.coords();
    let (res_x, res_y) = (pos_x as i32 + dir_x, pos_y as i32 + dir_y);
    if res_x < 0 || res_y < 0 {
        panic!("Moving out of bounds");
    }
    let (res_x, res_y) = (res_x as usize, res_y as usize);
    if current == '@'
        || (current == '[' && dir == Direction::Left)
        || (current == '[' && dir == Direction::Right)
        || (current == ']' && dir == Direction::Left)
        || (current == ']' && dir == Direction::Right)
    {
        move_pt2(res_x, res_y, dir, current, vec, false);
        *vec.get_mut(res_y).unwrap().get_mut(res_x).unwrap() = current;
        *vec.get_mut(pos_y).unwrap().get_mut(pos_x).unwrap() = char_to_put;
        return;
    }
    move_pt2(res_x, res_y, dir, current, vec, false);
    *vec.get_mut(res_y).unwrap().get_mut(res_x).unwrap() = current;
    *vec.get_mut(pos_y).unwrap().get_mut(pos_x).unwrap() = char_to_put;

    if current == '[' && !ignore_neighbor {
        move_pt2(pos_x + 1, pos_y, dir, '.', vec, true);
    }
    if current == ']' && !ignore_neighbor {
        move_pt2(pos_x - 1, pos_y, dir, '.', vec, true);
    }
}

fn can_move_pt2(
    pos_x: usize,
    pos_y: usize,
    dir: Direction,
    vec: &Vec<Vec<char>>,
    ignore_neighbor: bool,
) -> bool {
    trace!("can_move_pt2(pos_x: {pos_x}: usize, pos_y: {pos_y}, dir: {dir:?}, vec)");
    match vec.get(pos_y) {
        Some(line) => match line.get(pos_x) {
            Some(&ch) => match (ch, dir) {
                ('#', _) => false,
                ('.', _) => true,
                ('@', _)
                | ('[', Direction::Left)
                | ('[', Direction::Right)
                | (']', Direction::Left)
                | (']', Direction::Right) => {
                    let (dir_x, dir_y) = dir.coords();
                    let (res_x, res_y) = (pos_x as i32 + dir_x, pos_y as i32 + dir_y);
                    if res_x < 0 || res_y < 0 {
                        false
                    } else {
                        can_move_pt2(res_x as usize, res_y as usize, dir, vec, false)
                    }
                }
                // recursively check the direction + our box counter-part in the same direction.
                ('[', Direction::Up) | ('[', Direction::Down) => {
                    let (_, dir_y) = dir.coords();
                    let res_y = pos_y as i32 + dir_y;
                    let (res_x1, res_x2) = (pos_x, pos_x + 1);
                    if res_y < 0 {
                        false
                    } else {
                        can_move_pt2(res_x1 as usize, res_y as usize, dir, vec, false)
                            && (ignore_neighbor
                                || can_move_pt2(res_x2 as usize, pos_y, dir, vec, true))
                    }
                }
                (']', Direction::Up) | (']', Direction::Down) => {
                    let (_, dir_y) = dir.coords();
                    let res_y = pos_y as i32 + dir_y;
                    let (res_x1, res_x2) = (pos_x, pos_x - 1);
                    if res_y < 0 {
                        false
                    } else {
                        can_move_pt2(res_x1 as usize, res_y as usize, dir, vec, false)
                            && (ignore_neighbor
                                || can_move_pt2(res_x2 as usize, pos_y, dir, vec, true))
                    }
                }
                unknown_combo => panic!("Unrecognized can_move combo called {unknown_combo:?}"),
            },
            None => false,
        },
        None => false,
    }
}

fn find_guard(map_vec: &Vec<Vec<char>>) -> (usize, usize) {
    let robot_pos = map_vec
        .iter()
        .enumerate()
        .map(|(index, line)| (index, line.iter().position(|c| *c == '@')))
        .filter(|(_outer_index, inner_result)| inner_result.is_some())
        .next();

    match robot_pos {
        Some((outer_index, inner_result)) => match inner_result {
            Some(inner_index) => {
                return (inner_index, outer_index);
            }
            None => panic!("Couldn't find start_x"),
        },
        None => panic!("Couldn't find start_y"),
    }
}

fn parse_input_pt1(file_contents: String) -> (Vec<Vec<char>>, Vec<Direction>, (usize, usize)) {
    let mut parts = file_contents.split("\r\n\r\n");

    let map_vec: Vec<Vec<char>> = parts
        .next()
        .expect("Map part not found")
        .split("\r\n")
        .map(|line| line.chars().collect())
        .collect();
    let directions: Vec<Direction> = parts
        .next()
        .expect("directions not found")
        .chars()
        .filter(|&ch| ch != '\r' && ch != '\n')
        .map(|ch| match ch {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '>' => Direction::Right,
            '<' => Direction::Left,
            c => panic!("Unrecognized direction \"{c}\""),
        })
        .collect();

    let guard = find_guard(&map_vec);

    return (map_vec, directions, guard);
}

#[allow(dead_code)]
fn part_1(file_contents: String) {
    let (mut map_vec, directions, (mut guard_x, mut guard_y)) = parse_input_pt1(file_contents);
    // part 1.
    for (dir_idx, dir) in directions.into_iter().enumerate() {
        if can_move_pt1(guard_x, guard_y, dir, &map_vec) {
            move_pt1(guard_x, guard_y, dir, '.', &mut map_vec);
            let (dir_x, dir_y) = dir.coords();
            let (res_x, res_y) = (guard_x as i32 + dir_x, guard_y as i32 + dir_y);
            (guard_x, guard_y) = (res_x as usize, res_y as usize);
        }
        debug!("Step #{dir_idx} - Moved {dir:?}");
        for line in map_vec.iter() {
            debug!("{}", line.iter().collect::<String>());
        }
    }

    let mut sum = 0;
    for (line_idx, line) in map_vec.into_iter().enumerate() {
        for (ch_idx, ch) in line.into_iter().enumerate() {
            if ch == 'O' {
                sum += 100 * line_idx + ch_idx;
            }
        }
    }
    info!("Sum of all boxes' GPS coords: {sum}");
}

// This will swap this and next until it hits a wall or out of bounds.
// Swap next with current
fn move_pt1(
    pos_x: usize,
    pos_y: usize,
    dir: Direction,
    char_to_put: char,
    vec: &mut Vec<Vec<char>>,
) {
    // This represents the next character.
    let current = match vec.get(pos_y) {
        Some(line) => match line.get(pos_x) {
            Some(&ch) => ch,
            None => panic!("Moving an out of bounds point.x"),
        },
        None => panic!("Moving an out of bounds point.y"),
    };
    if current == '.' {
        *vec.get_mut(pos_y).unwrap().get_mut(pos_x).unwrap() = char_to_put;
        return;
    }
    if current == '#' {
        panic!("Tried to move a wall");
    }
    if current == '@' || current == 'O' {
        let (dir_x, dir_y) = dir.coords();
        let (res_x, res_y) = (pos_x as i32 + dir_x, pos_y as i32 + dir_y);
        if res_x < 0 || res_y < 0 {
            panic!("Moving out of bounds");
        } else {
            let (res_x, res_y) = (res_x as usize, res_y as usize);
            move_pt1(res_x, res_y, dir, current, vec);
            *vec.get_mut(res_y).unwrap().get_mut(res_x).unwrap() = current;
            *vec.get_mut(pos_y).unwrap().get_mut(pos_x).unwrap() = char_to_put;
            return;
        }
    }
    panic!("Tried to move unrecognized symbol");
}

fn can_move_pt1(pos_x: usize, pos_y: usize, dir: Direction, vec: &Vec<Vec<char>>) -> bool {
    match vec.get(pos_y) {
        Some(line) => match line.get(pos_x) {
            Some(&ch) => match ch {
                '#' => false,
                'O' | '@' => {
                    let (dir_x, dir_y) = dir.coords();
                    let (res_x, res_y) = (pos_x as i32 + dir_x, pos_y as i32 + dir_y);
                    if res_x < 0 || res_y < 0 {
                        false
                    } else {
                        can_move_pt1(res_x as usize, res_y as usize, dir, vec)
                    }
                }
                '.' => true,
                c => panic!("Unrecognized symbol: {c}"),
            },
            None => false,
        },
        None => false,
    }
}
