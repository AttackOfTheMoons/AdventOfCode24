use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use log::info;

use crate::Direction;

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\input\\week_3\\day18.txt";

const GRID_SIZE: usize = 70;

pub fn day18() {
    let file_contents =
        fs::read_to_string(INPUT_FILE).expect(format!("Could not read file {INPUT_FILE}").as_str());

    let corrupt_coords: Vec<(usize, usize)> = file_contents
        .trim()
        .split("\r\n")
        .map(|line| {
            let mut parts = line.split(",");
            (
                parts.next().unwrap().parse::<usize>().unwrap(),
                parts.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect();

    let mut lower_bound = 0;
    let mut upper_bound = corrupt_coords.len();
    let mut mid = (lower_bound + upper_bound) / 2;
    loop {
        let mut map_vec: Vec<Vec<char>> = (0..=GRID_SIZE)
            .map(|_| (0..=GRID_SIZE).map(|_| '.').collect())
            .collect();

        for (idx, &(coord_x, coord_y)) in corrupt_coords.iter().enumerate() {
            if idx == mid {
                break;
            }
            *map_vec.get_mut(coord_y).unwrap().get_mut(coord_x).unwrap() = '#';
        }
        let start_pos = (0, 0);
        let end_pos = (GRID_SIZE, GRID_SIZE);

        let end_score = find_shortest_path(map_vec, start_pos, end_pos);

        if end_score.is_some() {
            lower_bound = mid;
        } else {
            upper_bound = mid;
        }
        mid = (lower_bound + upper_bound) / 2;
        if lower_bound == mid {
            let maze_breaker = corrupt_coords.get(mid).unwrap();
            info!(
                "The byte to break the maze was: {},{}",
                maze_breaker.0, maze_breaker.1
            );
            break;
        }
    }
}

pub fn find_shortest_path(
    map_vec: Vec<Vec<char>>,
    start_pos: (usize, usize),
    end_pos: (usize, usize),
) -> Option<i32> {
    let mut to_visit = VecDeque::new();

    let start_score = 0;

    to_visit.push_back((start_pos, start_score));

    let mut visited = HashMap::new();

    let all_directions = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    while let Some((pos, curr_score)) = to_visit.pop_front() {
        match map_vec.get(pos.1) {
            Some(line) => match line.get(pos.0) {
                Some(&ch) => match ch {
                    '#' => continue,
                    _ => {}
                },
                None => continue,
            },
            None => continue,
        }
        if let Some(old_score) = visited.get_mut(&pos) {
            if *old_score <= curr_score {
                continue;
            } else {
                *old_score = curr_score;
            }
        } else {
            visited.insert(pos, curr_score);
        }

        for &dir in all_directions.iter() {
            if let Some(result) = dir.try_translate(pos) {
                let score_to_send = curr_score + 1;
                to_visit.push_back((result, score_to_send));
            }
        }
    }
    Some(visited.get(&end_pos)?.clone())
}
