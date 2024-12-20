use std::{collections::HashMap, fs};

use log::{debug, info, trace};

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\input\\week_3\\day19.txt";

pub fn day19() {
    let file_contents =
        fs::read_to_string(INPUT_FILE).expect(format!("Could not read file {INPUT_FILE}").as_str());
    let mut parts = file_contents.split("\r\n");
    let mut towels: Vec<_> = parts.next().unwrap().split(", ").collect();

    towels.sort_by(|a, b| b.len().cmp(&a.len()));
    let patterns: Vec<_> = parts.map(|s| s.trim()).filter(|s| !s.is_empty()).collect();

    trace!("towels: {towels:?}");
    trace!("patterns: {patterns:?}");

    let mut count = 0;

    // b, br, rw
    // brw
    let mut map = HashMap::new();
    for pattern in patterns {
        count += recursive_fn(pattern, &towels, 0, &mut map);
    }
    info!("{count} patterns are possible");
}

fn recursive_fn(
    pattern: &str,
    towels: &Vec<&str>,
    pos: usize,
    cnt_map: &mut HashMap<String, u64>,
) -> u64 {
    if pos == pattern.len() {
        return 1;
    }
    let mut valid_cnt = 0;
    if let Some(&value) = cnt_map.get(&pattern[pos..]) {
        return value;
    }
    for &towel in towels {
        if pos + towel.len() > pattern.len() {
            continue;
        }
        if &pattern[pos..pos + towel.len()] == towel {
            valid_cnt += recursive_fn(pattern, towels, pos + towel.len(), cnt_map);
        }
    }
    cnt_map.insert(pattern[pos..].to_string(), valid_cnt);
    return valid_cnt;
}
