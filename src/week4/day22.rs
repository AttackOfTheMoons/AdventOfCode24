use std::{
    collections::{HashMap, HashSet},
    fs,
};

use log::info;

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\input\\week_4\\day22.txt";

const PRUNE_NUMBER: i64 = 16_777_216;

const ITER_COUNT: usize = 2000;

pub fn day22() {
    let file_contents =
        fs::read_to_string(INPUT_FILE).expect(format!("Could not read file {INPUT_FILE}").as_str());
    part_one(&file_contents);
    part_two(&file_contents);
}

#[allow(dead_code)]
fn part_two(file_contents: &String) {
    let start_numbers = file_contents
        .split("\r\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.trim().parse::<i64>().unwrap_or_else(|_| {
                panic!("Couldn't parse '{s:?}' as i64");
            })
        })
        .collect::<Vec<_>>();

    let prices = start_numbers
        .iter()
        .map(|&secret| {
            let mut sequence = Vec::with_capacity(ITER_COUNT);
            let mut secret = secret;
            for _ in 0..ITER_COUNT {
                secret = next_random(secret);
                sequence.push(secret % 10);
            }
            sequence
        })
        .collect::<Vec<_>>();

    let mut changes = Vec::with_capacity(prices.len());
    for (sequence_idx, sequence) in prices.iter().enumerate() {
        let mut deltas = Vec::with_capacity(ITER_COUNT);
        let mut prev = start_numbers.get(sequence_idx).unwrap() % 10;
        for &num in sequence {
            deltas.push(num - prev);
            prev = num;
        }
        changes.push(deltas);
    }

    let mut seq_value_map: HashMap<&[i64], (i64, HashSet<usize>)> = HashMap::new();

    for i in 4..=ITER_COUNT {
        for (seq_idx, deltas) in changes.iter().enumerate() {
            let entry = seq_value_map
                .entry(&deltas[i - 4..i])
                .or_insert((0, HashSet::new()));

            if entry.1.contains(&seq_idx) {
                continue;
            }

            entry.0 += prices[seq_idx][i - 1];
            entry.1.insert(seq_idx);
        }
    }

    let max_seq = seq_value_map
        .iter()
        .max_by(|a, b| a.1 .0.cmp(&b.1 .0))
        .unwrap();

    info!(
        "Max value was from the sequence {:?} and was worth {} bananas",
        max_seq.0, max_seq.1 .0
    )
}

#[allow(dead_code)]
fn part_one(file_contents: &String) {
    let numbers_sum = file_contents
        .split("\r\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut secret = s.trim().parse::<i64>().unwrap_or_else(|_| {
                panic!("Couldn't parse '{s}' as i64");
            });
            for _ in 0..ITER_COUNT {
                secret = next_random(secret);
            }
            secret
        })
        .reduce(|s1, s2| s1 + s2);

    info!("Numbers_sum: {}", numbers_sum.unwrap());
}

pub fn next_random(mut current_random: i64) -> i64 {
    current_random = step1(current_random);
    current_random = step2(current_random);
    current_random = step3(current_random);
    current_random
}

// Calculate the result of multiplying the secret number by 64.
// Then, mix this result into the secret number. Finally, prune the secret number.
fn step1(mut secret_number: i64) -> i64 {
    secret_number = mix(secret_number, secret_number * 64);
    secret_number = prune(secret_number);
    secret_number
}

// To mix a value into the secret number, calculate the bitwise XOR of the given value and the secret number.
// Then, the secret number becomes the result of that operation.
// (If the secret number is 42 and you were to mix 15 into the secret number, the secret number would become 37.)
fn mix(secret_number: i64, number_to_mix_in: i64) -> i64 {
    secret_number ^ number_to_mix_in
}

// To prune the secret number, calculate the value of the secret number modulo 16777216.
// Then, the secret number becomes the result of that operation. (If the secret number is 100000000 and you were to prune the secret number, the secret number would become 16113920.)
fn prune(secret_number: i64) -> i64 {
    secret_number % PRUNE_NUMBER
}

// Calculate the result of dividing the secret number by 32.
// Round the result down to the nearest integer.
// Then, mix this result into the secret number. Finally, prune the secret number.
fn step2(mut secret_number: i64) -> i64 {
    secret_number = mix(secret_number, secret_number / 32);
    secret_number = prune(secret_number);
    secret_number
}

// Calculate the result of multiplying the secret number by 2048.
// Then, mix this result into the secret number. Finally, prune the secret number.
fn step3(mut secret_number: i64) -> i64 {
    secret_number = mix(secret_number, secret_number * 2048);
    secret_number = prune(secret_number);
    secret_number
}
