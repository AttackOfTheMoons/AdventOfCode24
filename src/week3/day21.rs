use core::num;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs,
    process::exit,
};

use log::{debug, info, trace};
use regex::Regex;

use crate::Direction;

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\input\\week_3\\day21.txt";

const CODE_PATTERN: &str = r"(\d+)A";

pub fn day21() {
    let file_contents =
        fs::read_to_string(INPUT_FILE).expect(format!("Could not read file {INPUT_FILE}").as_str());

    let standard_keyboard = Keyboard::new(
        4,
        3,
        vec!['7', '8', '9', '4', '5', '6', '1', '2', '3', '0', 'A'],
        HashSet::from([9]),
    );

    let arrow_keyboard = make_arrow_keyboard();

    let re = Regex::new(CODE_PATTERN).unwrap();

    let mut total = 0;
    for pattern in re.captures_iter(&file_contents) {
        if let Some(number_match) = pattern.get(1) {
            let original_seq = pattern.get(0).unwrap().as_str().chars().collect();

            let seq = standard_keyboard.find_sequence(original_seq, 2, &arrow_keyboard);
            // trace!("seq: {:?}", seq);
            let layer2 = arrow_keyboard.find_sequence(seq, 1, &arrow_keyboard);
            // trace!("layer2: {:?}", layer2);
            let layer3 = arrow_keyboard.find_sequence(layer2, 0, &arrow_keyboard);
            // trace!("layer3: {:?}", layer3);
            let digits_val = number_match.as_str().parse::<usize>().unwrap();
            total += digits_val * layer3.len();
            trace!(
                "{}A: {} * {digits_val}",
                &file_contents[number_match.range()],
                layer3.len()
            );
        }
    }
    info!("Complexity score = {total}");
}

struct Keyboard {
    key_paths: HashMap<(char, char), Vec<Vec<char>>>,
}

fn make_arrow_keyboard() -> Keyboard {
    Keyboard::new(2, 3, vec!['^', 'A', '<', 'v', '>'], HashSet::from([0]))
}

// This is what I like to call overengineering a problem.
// Probably not for any benefit.
impl Keyboard {
    pub fn find_sequence(
        &self,
        sequence: Vec<char>,
        layer: usize,
        arrow_keyboard: &Keyboard,
    ) -> Vec<char> {
        // a list of all possible paths that could be taken
        let mut possible_paths: Vec<Vec<char>> = Vec::new();

        let mut prev = 'A';
        for c in sequence {
            if let Some(paths) = self.key_paths.get(&(prev, c)) {
                // trace!("Handling {prev} -> {c}");
                // Last is always A. both previous path's last and current path's last
                if possible_paths.len() == 0 {
                    possible_paths.extend(paths.clone());
                    prev = c;
                    continue;
                }
                // There are 1 or more possible existing paths.

                let mut new_possible = HashSet::new();
                for path in paths {
                    let mut possible_paths_clone = possible_paths.clone();
                    for curr in possible_paths_clone.iter_mut() {
                        // trace!("{prev} -> {c} == {curr:?}");
                        curr.extend(path);
                    }
                    new_possible.extend(possible_paths_clone);
                }
                // trace!("new_possible: {new_possible:?}");

                possible_paths = Vec::from_iter(new_possible.into_iter());
                prev = c;
            } else {
                panic!("Missing a path from {prev} to {c}");
            }
        }
        // to pick the best possible path, simulate the number of layers left.
        if layer == 0 {
            // If this is the bottom layer, no need to simulate, just use the first one.
            // trace!("Returning here layer == 0");
            return possible_paths[0].clone();
        }

        let mut best_path: Option<(&Vec<char>, usize)> = None;

        for path in possible_paths.iter() {
            let mut test_path = path.clone();
            for i in (0..layer).rev() {
                // trace!("Testing {i} - {test_path:?}, ");
                test_path = arrow_keyboard.find_sequence(test_path, i, arrow_keyboard);
            }
            if let Some(best) = best_path {
                if best.1 > test_path.len() {
                    best_path = Some((path, test_path.len()));
                }
            } else {
                best_path = Some((path, test_path.len()));
            }
        }
        // trace!(
        //     "Returning here best path was {:?} out of {:?}",
        //     best_path.unwrap().0,
        //     possible_paths
        // );
        return best_path.unwrap().0.clone();
    }

    fn new(rows: usize, cols: usize, keys: Vec<char>, dead_indices: HashSet<usize>) -> Self {
        let mut key_map = Vec::new();
        let mut offset = 0;
        let mut key_indices = HashMap::new();
        for row_idx in 0..rows {
            let mut row = Vec::new();
            for col_idx in 0..cols {
                if dead_indices.contains(&(row_idx * cols + col_idx)) {
                    row.push('#');
                    offset += 1;
                    continue;
                }
                if let Some(&c) = keys.get(row_idx * cols + col_idx - offset) {
                    row.push(c);
                    key_indices.insert(c, (col_idx, row_idx));
                }
            }
            key_map.push(row);
        }
        let mut pairs = Vec::new();
        for row in key_map.iter() {
            for &ch in row.iter() {
                for row1 in key_map.iter() {
                    for &ch1 in row1.iter() {
                        if ch == '#' || ch1 == '#' {
                            continue;
                        }
                        pairs.push((ch, ch1));
                    }
                }
            }
        }
        let mut key_paths = HashMap::new();

        for (start, finish) in pairs {
            let (mut current_x, mut current_y): (usize, usize);
            match key_indices.get(&start) {
                None => panic!("{start} not added to key_indices"),
                Some(&(x, y)) => (current_x, current_y) = (x, y),
            }
            let (end_x, end_y): (usize, usize);
            match key_indices.get(&finish) {
                None => panic!("{finish} not added to key_indices"),
                Some(&(x, y)) => (end_x, end_y) = (x, y),
            }
            let mut sequence = Vec::new();
            while current_x != end_x || current_y != end_y {
                let dir = best_dir((current_x, current_y), (end_x, end_y));
                sequence.push(char::from(&dir));
                if let Some((res_x, res_y)) = dir.try_translate((current_x, current_y)) {
                    (current_x, current_y) = (res_x, res_y);
                } else {
                    panic!("tried to go out of bounds from ({current_x}, {current_y}) to ({end_x}, {end_y}) moving ({dir:?})");
                }
            }
            // trace!("{sequence:?} was the valid one generated for {start} -> {finish}");
            // This is the number of least # of steps to get from start to finish.
            // Now let's generate the mutations and test each of them.
            let mut valid_sequences = HashSet::new();
            'outer_loop: for j in 0..sequence.len() {
                let mut new_seq = Vec::new();
                for i in 0..sequence.len() {
                    new_seq.push(sequence[(j + i) % sequence.len()]);
                }
                if valid_sequences.contains(&new_seq) {
                    continue 'outer_loop;
                }
                // trace!("{new_seq:?} is a mutation for  {start} -> {finish}");

                let mut current = key_indices[&start];
                for button in new_seq.clone() {
                    if button == 'A' {
                        continue;
                    }
                    if let Some(current_res) = Direction::from(button).try_translate(current) {
                        if let Some(row) = key_map.get(current.1) {
                            if let Some(ch) = row.get(current.0) {
                                if *ch == '#' {
                                    continue 'outer_loop;
                                } else {
                                    current = current_res;
                                }
                            } else {
                                continue 'outer_loop;
                            }
                        } else {
                            continue 'outer_loop;
                        }
                    } else {
                        continue 'outer_loop;
                    }
                }
                if let Some(row) = key_map.get(current.1) {
                    if let Some(ch) = row.get(current.0) {
                        if *ch != finish {
                            continue 'outer_loop;
                        }
                    } else {
                        continue 'outer_loop;
                    }
                } else {
                    continue 'outer_loop;
                }
                valid_sequences.insert(new_seq);
            }

            let mut valid_sequences = Vec::from_iter(valid_sequences);
            for seq in valid_sequences.iter_mut() {
                seq.push('A');
            }
            if valid_sequences.len() == 0 {
                valid_sequences.push(vec!['A']);
            }

            // trace!("key_paths.insert(({start}, {finish}), {valid_sequences:?})");
            key_paths.insert((start, finish), valid_sequences);
        }
        Self { key_paths }
    }
}

impl From<char> for Direction {
    fn from(ch: char) -> Self {
        match ch {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            ch => panic!("tried to make a direction from '{ch}'"),
        }
    }
}

impl From<&Direction> for char {
    fn from(dir: &Direction) -> Self {
        match dir {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

fn best_dir((pt1_x, pt1_y): (usize, usize), (pt2_x, pt2_y): (usize, usize)) -> Direction {
    if pt2_x > pt1_x {
        return Direction::Right;
    }
    if pt2_y > pt1_y {
        return Direction::Down;
    }
    if pt1_y > pt2_y {
        return Direction::Up;
    }
    return Direction::Left;
}
