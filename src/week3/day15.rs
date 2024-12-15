use std::fs;

use log::{debug, info, trace};

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\day15.txt";

pub fn day15() {
    let file_contents =
        fs::read_to_string(INPUT_FILE).expect(format!("Could not read file {INPUT_FILE}").as_str());
}
