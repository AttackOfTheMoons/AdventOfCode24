use std::fs;

use log::{debug, info, trace};

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\input\\week_4\\day25.txt";

pub fn day25() {
    let file_contents =
        fs::read_to_string(INPUT_FILE).expect(format!("Could not read file {INPUT_FILE}").as_str());

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for i in file_contents.split("\r\n\r\n") {
        if i.starts_with("#####") {
            let mut found = vec![None, None, None, None, None];
            for (idx, line) in i.split("\r\n").enumerate() {
                for (c_id, c) in line.chars().enumerate() {
                    if found[c_id].is_none() && c == '.' {
                        found[c_id] = Some(idx - 1);
                    }
                }
            }
            let found = found.into_iter().filter_map(|x| x).collect::<Vec<_>>();
            locks.push(found);
        } else {
            // key
            let mut found = vec![None, None, None, None, None];
            for (idx, line) in i.split("\r\n").enumerate() {
                for (c_id, c) in line.chars().enumerate() {
                    if found[c_id].is_none() && c == '#' {
                        found[c_id] = Some(idx - 1);
                    }
                }
            }
            let found = found.into_iter().filter_map(|x| x).collect::<Vec<_>>();
            keys.push(found);
        }
    }
    let mut sum = 0;
    for lock in locks {
        'key_loop: for key in keys.iter() {
            for (idx, &i) in key.iter().enumerate() {
                if lock[idx] > i {
                    continue 'key_loop;
                }
            }
            trace!("Lock: {lock:?} and key: {key:?} fit");
            sum += 1;
        }
    }
    info!("There are {sum} total combinations to try!");
}
