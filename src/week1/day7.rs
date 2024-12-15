use std::fs;

use log::info;

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\input\\week_1\\day7.txt";

enum OperationType {
    Add,
    Multiply,
    Concatenate,
}

pub fn day7() {
    let file_contents =
        fs::read_to_string(INPUT_FILE).expect(format!("Could not read file {INPUT_FILE}").as_str());

    let line_vec: Vec<_> = file_contents
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|line| {
            let mut parts = line.split(": ");
            let subtotal = parts
                .next()
                .expect("Couldn't unwrap subtotal")
                .parse::<u64>()
                .expect("Couldn't parse subtotal");

            let nums: Vec<u64> = parts
                .next()
                .expect("Couldn't unwrap nums")
                .trim()
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|x| {
                    x.parse::<u64>()
                        .expect(format!("Couldn't parse num '{x}'").as_str())
                })
                .collect();
            (subtotal, nums)
        })
        .collect();

    let mut sum = 0;
    for (total, nums) in line_vec {
        // Recursively check both paths, from adding or multiplying each number to the current sum
        if check_val(OperationType::Add, 0, 0, &nums, total)
            || check_val(OperationType::Multiply, 1, 0, &nums, total)
        {
            sum += total;
        }
    }
    info!("Sum: {sum}");
}

// if not adding, multiply
fn check_val(
    operation: OperationType,
    curr_value: u64,
    start_idx: usize,
    nums: &Vec<u64>,
    goal: u64,
) -> bool {
    if start_idx == nums.len() {
        return curr_value == goal;
    }

    let next_idx = start_idx + 1;
    let curr_num = nums.get(start_idx).unwrap();
    match operation {
        OperationType::Add => {
            let add_result = curr_value + curr_num;
            check_val(OperationType::Add, add_result, next_idx, nums, goal)
                || check_val(OperationType::Multiply, add_result, next_idx, nums, goal)
                || check_val(OperationType::Concatenate, add_result, next_idx, nums, goal)
        }
        OperationType::Multiply => {
            let mul_result = curr_value * curr_num;
            check_val(OperationType::Add, mul_result, next_idx, nums, goal)
                || check_val(OperationType::Multiply, mul_result, next_idx, nums, goal)
                || check_val(OperationType::Concatenate, mul_result, next_idx, nums, goal)
        }
        OperationType::Concatenate => {
            let concat_result = curr_value
                * u64::pow(10, f64::log10(*curr_num as f64).floor() as u32 + 1)
                + curr_num;
            // trace!("{curr_value} || {curr_num} = {concat_result}");
            check_val(OperationType::Add, concat_result, next_idx, nums, goal)
                || check_val(OperationType::Multiply, concat_result, next_idx, nums, goal)
                || check_val(
                    OperationType::Concatenate,
                    concat_result,
                    next_idx,
                    nums,
                    goal,
                )
        }
    }
}
