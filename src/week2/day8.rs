use std::{
    collections::{HashMap, HashSet},
    fs,
};

use log::{debug, info};

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\day8.txt";

pub fn day8() {
    let file_contents =
        fs::read_to_string(INPUT_FILE).expect(format!("Could not read file {INPUT_FILE}").as_str());

    let outer_idx_len = file_contents.trim().split("\n").count();
    let inner_idx_len = file_contents
        .trim()
        .split("\n")
        .next()
        .unwrap_or_else(|| "")
        .trim()
        .len();

    let mut positions_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (line_idx, line) in file_contents.trim().split("\n").enumerate() {
        for (ch_idx, ch) in line.trim().char_indices() {
            if ch == '.' {
                continue;
            }
            positions_map
                .entry(ch)
                .or_insert(vec![])
                .push((ch_idx, line_idx));
        }
    }
    // The key is irrelevant for the antinodes.
    let mut antinodes = HashSet::new();

    debug!("Grid size = {inner_idx_len} x {outer_idx_len}");

    for (_key, positions) in positions_map {
        for (i, el1) in positions.iter().enumerate() {
            for el2 in &positions[i + 1..] {
                // The change for part 2 vs part1 is the original nodes are also antinodes + needs the while loop to add all
                let el1 = (el1.0 as i32, el1.1 as i32);
                let el2 = (el2.0 as i32, el2.1 as i32);
                let diff_0 = el1.0 - el2.0;
                let diff_1 = el1.1 as i32 - el2.1 as i32;
                let mut antinode_1 = (el1.0, el1.1);
                let mut antinode_2 = (el2.0, el2.1);
                while node_in_bounds(antinode_1, inner_idx_len - 1, outer_idx_len - 1) {
                    antinodes.insert(antinode_1);
                    antinode_1 = (antinode_1.0 + diff_0, antinode_1.1 + diff_1);
                }
                while node_in_bounds(antinode_2, inner_idx_len - 1, outer_idx_len - 1) {
                    antinodes.insert(antinode_2);
                    antinode_2 = (antinode_2.0 - diff_0, antinode_2.1 - diff_1);
                }
            }
        }
    }

    // printing the output grid to console
    // let mut file_contents_arr: Vec<Vec<char>> = file_contents
    //     .trim()
    //     .split("\n")
    //     .map(|line| line.trim().chars().collect())
    //     .collect();

    // for (x, y) in antinodes {
    //     let ch = file_contents_arr
    //         .get_mut(y as usize)
    //         .unwrap()
    //         .get_mut(x as usize)
    //         .unwrap();
    //     if *ch == '.' {
    //         *ch = '#';
    //     }
    // }

    // println!(
    //     "{}",
    //     file_contents_arr
    //         .iter()
    //         .map(|line_vec| { line_vec.into_iter().collect::<String>() })
    //         .collect::<Vec<String>>()
    //         .join("\n")
    // );

    info!("There were {} antinodes", antinodes.len());
}

fn node_in_bounds((node_x, node_y): (i32, i32), upper_x: usize, upper_y: usize) -> bool {
    return node_x >= 0 && node_y >= 0 && node_x <= (upper_x as i32) && node_y <= (upper_y as i32);
}
