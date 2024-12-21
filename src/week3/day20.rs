use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs, i32, vec,
};

use log::{debug, info, trace};

use crate::{find_single_instance_of_char, Direction};

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\input\\week_3\\day20.txt";

const CHEAT_LENGTH: i32 = 20;

pub fn day20() {
    let file_contents =
        fs::read_to_string(INPUT_FILE).expect(format!("Could not read file {INPUT_FILE}").as_str());
    let map_vec: Vec<_> = file_contents
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect();

    let start_pos = find_single_instance_of_char(&map_vec, 'S');

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

    let mut vec_map_weights = Vec::with_capacity(map_vec.len());
    for line in map_vec.iter() {
        let mut new_vec = Vec::new();
        for _ in line.iter() {
            new_vec.push(-1);
        }
        vec_map_weights.push(new_vec);
    }

    for ((pos_x, pos_y), value) in visited {
        unsafe {
            *vec_map_weights
                .get_unchecked_mut(pos_y)
                .get_unchecked_mut(pos_x) = value;
        }
    }

    let mut cheat_map = HashMap::new();
    for (line_idx, line) in vec_map_weights.iter().enumerate() {
        for (idx, &_i) in line.iter().enumerate() {
            search_depth(
                (idx, line_idx),
                CHEAT_LENGTH,
                &vec_map_weights,
                &mut cheat_map,
            );
        }
    }
    let mut cheats: HashMap<((usize, usize), (usize, usize)), i32> = HashMap::new();

    for (((pos_x, pos_y), _depth), value) in cheat_map {
        let start_value = *vec_map_weights.get(pos_y).unwrap().get(pos_x).unwrap();
        if start_value == -1 {
            continue;
        }
        for &(reach_x, reach_y) in value.iter() {
            let reach_value = *vec_map_weights.get(reach_y).unwrap().get(reach_x).unwrap();
            let cheat_value = reach_value
                - start_value
                - (pos_x.abs_diff(reach_x) + pos_y.abs_diff(reach_y)) as i32;
            *cheats
                .entry(((pos_x, pos_y), (reach_x, reach_y)))
                .or_default() = cheat_value;
        }
    }
    let mut cheat_hashmap: HashMap<i32, i32> = HashMap::new();
    for (_pts, &value) in cheats.iter().filter(|&c| *c.1 > 0) {
        trace!("{_pts:?} == {value}");
        *cheat_hashmap.entry(value).or_default() += 1;
    }

    let mut sorted_cheats = cheat_hashmap.into_iter().collect::<Vec<_>>();
    sorted_cheats.sort();
    for (value, count) in sorted_cheats {
        debug!("There are {count} cheats that save {value} picoseconds");
    }

    let cheat_count = cheats.into_iter().filter(|a| a.1 >= 100).count();
    info!("There are {cheat_count} cheats that would save 100 or more picoseconds");
}

fn search_depth(
    (pos_x, pos_y): (usize, usize),
    depth: i32,
    vec_map_weights: &Vec<Vec<i32>>,
    cheat_map: &mut HashMap<((usize, usize), i32), HashSet<(usize, usize)>>,
) {
    if let Some(line) = vec_map_weights.get(pos_y) {
        if let Some(&i) = line.get(pos_x) {
            if i != -1 {
                cheat_map.insert(((pos_x, pos_y), 0), HashSet::from([(pos_x, pos_y)]));
            }
        } else {
            // cheats cannot go out of bounds.
            return;
        }
    } else {
        // cheats cannot go out of bounds.
        return;
    }
    if depth == 0 {
        return;
    }
    let mut results = HashSet::new();
    for dir in vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        if let Some(result) = dir.try_translate((pos_x, pos_y)) {
            if let Some(reachable_points) = cheat_map.get(&(result, depth - 1)) {
                for &(point_x, point_y) in reachable_points.iter() {
                    if let Some(line) = vec_map_weights.get(point_y) {
                        if let Some(&_value) = line.get(point_x) {
                            results.insert((point_x, point_y));
                        }
                    }
                }
                // Don't re-search a neighbor that has been searched already.
                continue;
            }
            search_depth(result, depth - 1, vec_map_weights, cheat_map);
            // lets try and reach ()
            for depth in 0..depth {
                if let Some(reachable_points) = cheat_map.get(&(result, depth)) {
                    for &(point_x, point_y) in reachable_points.iter() {
                        if let Some(line) = vec_map_weights.get(point_y) {
                            if let Some(&_value) = line.get(point_x) {
                                results.insert((point_x, point_y));
                            }
                        }
                    }
                }
            }
        }
    }
    cheat_map.insert(((pos_x, pos_y), depth), results);
}
