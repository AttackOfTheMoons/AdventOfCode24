use indicatif::ProgressBar;
use std::{
    collections::{BTreeSet, HashMap},
    fs,
};

use log::{debug, info, trace};

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\day9.txt";

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Range {
    start_idx: u32,
    len: u32,
}

impl Range {
    fn new(start_idx: u32, len: u32) -> Range {
        Range { start_idx, len }
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start_idx.cmp(&other.start_idx)
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.start_idx.partial_cmp(&other.start_idx)
    }
}

pub fn day9() {
    let file_contents =
        fs::read_to_string(INPUT_FILE).expect(format!("Could not read file {INPUT_FILE}").as_str());

    // part1_moves(result_memory);

    let chars: Vec<_> = file_contents.trim().chars().collect();

    let mut id = 0;

    let mut memory_map = HashMap::new();
    let mut free_memory = BTreeSet::new();
    let mut result_memory = 0;
    for index in 0..((chars.len() + 1) / 2) {
        let block = chars.get(2 * index).unwrap_or(&'0').to_digit(10).unwrap();
        let free = chars
            .get(2 * index + 1)
            .unwrap_or(&'0')
            .to_digit(10)
            .unwrap();
        memory_map.insert(id, (result_memory, block));
        result_memory += block;
        // How to represent free memory? maybe with ranges?
        free_memory.insert(Range::new(result_memory, free));
        result_memory += free;
        id += 1;
    }

    info!("Moving memory... ");

    let bar = ProgressBar::new(id as u64);
    for i in bar.wrap_iter((0..id).rev()) {
        let (start_idx, block_size) = *memory_map.get(&i).unwrap();
        trace!("File #{i} currently sits at {start_idx} ({block_size} long)");

        if let Some(&free) = free_memory
            .iter()
            .find(|&&r| r.len >= block_size && r.start_idx < start_idx)
        {
            free_memory.remove(&free);
            trace!(
                "Found open spot for {i} which is {block_size} long at {}",
                free.start_idx
            );
            if free.len > block_size {
                free_memory.insert(Range::new(
                    free.start_idx + block_size,
                    free.len - block_size,
                ));
            }

            memory_map.remove(&i);
            memory_map.insert(i, (free.start_idx, block_size));
            free_memory.insert(Range::new(start_idx, block_size));
        }
    }
    debug!("Memory_map: {memory_map:?}");

    let mut check_sum: u64 = 0;

    for (key, (start_idx, len)) in memory_map {
        for i in start_idx..(start_idx + len) {
            check_sum += (i * key) as u64;
        }
    }

    info!("check_sum: {check_sum:?}");
}

#[allow(dead_code)]
fn part1_moves(file_contents: String) {
    let chars: Vec<_> = file_contents.trim().chars().collect();

    let mut result_memory: Vec<_> = Vec::new();

    let mut id = 0;
    for index in 0..((chars.len() + 1) + 1 / 2) {
        let block = chars.get(2 * index).unwrap_or(&'0').to_digit(10).unwrap();
        let free = chars
            .get(2 * index + 1)
            .unwrap_or(&'0')
            .to_digit(10)
            .unwrap();
        for _ in 0..block {
            result_memory.push(Some(id));
        }
        for _ in 0..free {
            result_memory.push(None);
        }
        id += 1;
    }

    let mut i = 0;
    let mut j = result_memory.len() - 1;
    while i < j {
        let lower = result_memory.get(i).unwrap();
        if lower.is_some() {
            i += 1;
            continue;
        }
        let upper = result_memory.get(j).unwrap();
        if upper.is_none() {
            j -= 1;
            continue;
        }

        result_memory.swap(i, j);
        i += 1;
        j -= 1;
    }

    let mut sum: u64 = 0;
    for (index, value) in result_memory.into_iter().enumerate() {
        if value.is_none() {
            break;
        }
        sum += (index as u64) * value.unwrap();
    }
    info!("Sum: {sum}");
}
