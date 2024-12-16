use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use log::{info, trace};

use crate::{find_single_instance_of_char, Direction};

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\input\\week_3\\day16.txt";

pub fn day16() {
    let file_contents =
        fs::read_to_string(INPUT_FILE).expect(format!("Could not read file {INPUT_FILE}").as_str());

    let map_vec: Vec<Vec<char>> = file_contents
        .split("\r\n")
        .map(|line| line.chars().collect())
        .collect();

    let start_pos = find_single_instance_of_char(&map_vec, 'S');

    let mut to_visit = VecDeque::new();

    let start_score = 0;

    to_visit.push_back(((start_pos, Direction::Right), start_score));

    let mut visited = HashMap::new();

    let all_directions = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    while let Some((state, curr_score)) = to_visit.pop_front() {
        match map_vec.get(state.0 .1) {
            Some(line) => match line.get(state.0 .0) {
                Some(&ch) => match ch {
                    '#' => continue,
                    _ => {}
                },
                None => continue,
            },
            None => continue,
        }
        if let Some(old_score) = visited.get_mut(&state) {
            if *old_score <= curr_score {
                continue;
            } else {
                *old_score = curr_score;
            }
        } else {
            visited.insert(state, curr_score);
        }

        for &dir in all_directions.iter() {
            if let Some(result) = dir.try_translate(state.0) {
                let mut score_to_send = curr_score + 1;
                if dir != state.1 {
                    score_to_send += 1000;
                }
                to_visit.push_back(((result, dir), score_to_send));
            }
        }
    }
    let end_pos = find_single_instance_of_char(&map_vec, 'E');

    let end_score = all_directions
        .iter()
        .filter_map(|&dir| visited.get(&(end_pos, dir)))
        .min()
        .unwrap();
    info!("Reindeer score: {}", end_score);
}

// recursive DFS approach stack overflows on large input.
fn first_attempt(map_vec: &Vec<Vec<char>>) {
    let current_pos = find_single_instance_of_char(&map_vec, 'S');

    let mut visited = HashMap::new();

    // The reindeer starts facing east is a constraint from the problem.
    let score = search(current_pos, Direction::Right, 0, &mut visited, &map_vec).unwrap();
    info!("Reindeer score: {score}");
}

fn search(
    current_pos: (usize, usize),
    facing: Direction,
    curr_score: u64,
    visited: &mut HashMap<(usize, usize), u64>,
    map_vec: &Vec<Vec<char>>,
) -> Option<u64> {
    trace!("search({current_pos:?}, {facing:?}, {curr_score})");
    // This does our bounds checking + if we are at the end + walls.
    match map_vec.get(current_pos.1) {
        Some(line) => match line.get(current_pos.0) {
            Some(&ch) => match ch {
                '#' => return None,
                'E' => return Some(curr_score),
                _ => {}
            },
            None => return None,
        },
        None => return None,
    }
    // If we previously reached the current location with a better score, this current path isn't worth continuing.
    match visited.get_mut(&current_pos) {
        Some(score) => {
            if *score > curr_score {
                *score = curr_score;
            } else {
                return None;
            }
        }
        None => {
            visited.insert(current_pos, curr_score);
        }
    }
    let mut best_score = None;
    for dir in vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        if let Some(result) = dir.try_translate(current_pos) {
            let mut score_to_send = curr_score + 1;
            if dir != facing {
                score_to_send += 1000;
            }
            if let Some(new_score) = search(result, dir, score_to_send, visited, map_vec) {
                match best_score {
                    Some(score) => {
                        if new_score < score {
                            best_score = Some(new_score)
                        }
                    }
                    None => best_score = Some(new_score),
                }
            }
        }
    }
    return best_score;
}
