use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;
use log::{debug, info, trace};
use regex::Regex;

use crate::Direction;

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\input\\week_3\\day21.txt";

const CODE_PATTERN: &str = r"(\d+)A";

const LAYERS: usize = 25;

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
    let mut optimal_paths: HashMap<(char, char), (usize, Vec<char>)> = HashMap::new();
    let warmup = (1..=2)
        .map(|_| 0..=10)
        .multi_cartesian_product()
        .flatten()
        .map(|c| match c {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            10 => 'A',
            _ => panic!(),
        })
        .collect::<Vec<_>>();

    // Warmup cache?
    let _ = standard_keyboard.find_sequence(warmup, 5, &arrow_keyboard, &mut optimal_paths);

    for pattern in re.captures_iter(&file_contents) {
        if let Some(number_match) = pattern.get(1) {
            let original_seq = pattern.get(0).unwrap().as_str().chars().collect();
            let mut seq = standard_keyboard.find_sequence(
                original_seq,
                LAYERS,
                &arrow_keyboard,
                &mut optimal_paths,
            );
            for i in (0..LAYERS - 1).rev() {
                seq = arrow_keyboard.find_sequence(seq, i, &arrow_keyboard, &mut optimal_paths);
            }
            let digits_val = number_match.as_str().parse::<usize>().unwrap();
            total += digits_val * seq.len();
            debug!(
                "{}A: {} * {digits_val}",
                &file_contents[number_match.range()],
                seq.len()
            );
        }
    }
    trace!("{optimal_paths:?}");
    info!("Complexity score = {total}");
}

struct Keyboard {
    key_paths: HashMap<(char, char), Vec<Vec<char>>>,
}

fn make_arrow_keyboard() -> Keyboard {
    Keyboard::new(2, 3, vec!['^', 'A', '<', 'v', '>'], HashSet::from([0]))
}

impl Keyboard {
    pub fn find_sequence(
        &self,
        sequence: Vec<char>,
        layer: usize,
        arrow_keyboard: &Keyboard,
        optimal_paths: &mut HashMap<(char, char), (usize, Vec<char>)>,
    ) -> Vec<char> {
        let mut result: Vec<char> = Vec::new();

        let mut prev = 'A';
        for c in sequence {
            if prev == c {
                result.push('A');
                continue;
            }
            if let Some(best_path) = optimal_paths.get(&((prev, c))) {
                if best_path.0 >= layer || best_path.0 > 3 {
                    // if we already found it at a depth further than 3, surely its good enough.
                    result.extend(best_path.1.iter());
                    prev = c;
                    continue;
                }
                trace!("manually calculating best path for {prev}, {c} at depth {layer} since highest cache was {}", best_path.0);
            }
            if let Some(paths) = self.key_paths.get(&(prev, c)) {
                // Not necessary to optimize the last layer.
                if layer == 0 {
                    result.extend(paths.get(0).unwrap());
                    prev = c;
                    continue;
                }
                // else:
                trace!("manually calculating best path for {prev}, {c} at depth {layer}");
                let mut best_path: Option<(&Vec<char>, usize)> = None;
                for path in paths.iter() {
                    let mut test_path = path.clone();
                    for i in (0..layer).rev() {
                        // trace!("Testing {i} - {test_path:?}, ");
                        test_path = arrow_keyboard.find_sequence(
                            test_path,
                            i,
                            arrow_keyboard,
                            optimal_paths,
                        );
                    }
                    if let Some(best) = best_path {
                        if best.1 > test_path.len() {
                            best_path = Some((path, test_path.len()));
                        }
                    } else {
                        best_path = Some((path, test_path.len()));
                    }
                }
                assert_ne!(best_path, None);

                let best_path = best_path.unwrap().0;
                optimal_paths.insert((prev, c), (layer, best_path.clone()));
                result.extend(best_path);
                prev = c;
            } else {
                panic!("Missing a path from {prev} to {c}");
            }
        }
        return result;
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
