use std::{collections::HashMap, fs};

use log::{info, trace};

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\input\\week_2\\day11.txt";

// Change this for part 1 vs part 2.
const BLINK_COUNT: usize = 75;

pub fn day11() {
    let file_contents =
        fs::read_to_string(INPUT_FILE).expect(format!("Could not read file {INPUT_FILE}").as_str());

    // parse input as a vec of u64.
    let num_vec = file_contents
        .split(" ")
        .map(|num_str| num_str.trim().parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut sum = 0;
    let mut memos = HashMap::new();
    for i in num_vec {
        sum += blink_result(i, BLINK_COUNT, &mut memos);
    }
    info!("Stone count: {sum}");
}

fn blink_result(stone_num: u64, blinks_left: usize, memos: &mut HashMap<(u64, usize), u64>) -> u64 {
    if blinks_left == 0 {
        return 1;
    }

    if let Some(&result) = memos.get(&(stone_num, blinks_left)) {
        trace!("Cache hit: {stone_num} => {result}");
        return result;
    }

    if stone_num == 0 {
        let result = blink_result(1, blinks_left - 1, memos);
        trace!("{stone_num} was 0 => 1");
        memos.insert((stone_num, blinks_left), result);
        return result;
    }

    let digits = stone_num.checked_ilog10().unwrap_or(0) + 1;
    if digits % 2 == 0 {
        let power_10 = 10_u64.pow(digits / 2);
        let left_half = stone_num / power_10;
        let right_half = stone_num - left_half * power_10;
        let result = blink_result(left_half, blinks_left - 1, memos)
            + blink_result(right_half, blinks_left - 1, memos);
        trace!("{stone_num} SPLIT => {left_half}, {right_half}");
        memos.insert((stone_num, blinks_left), result);
        return result;
    }

    let result = blink_result(stone_num * 2024, blinks_left - 1, memos);
    trace!("{stone_num} * 2024 => {}", stone_num * 2024);
    memos.insert((stone_num, blinks_left), result);
    return result;
}
