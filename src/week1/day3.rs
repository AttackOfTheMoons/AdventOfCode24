use log::{debug, info};
use std::fs;

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\day3.txt";

trait Evaluable {
    fn evaluate(&self) -> u32;
}

struct State {
    sequence_index: usize,
    num_1: u32,
    num_2: u32,
}

impl Evaluable for State {
    fn evaluate(&self) -> u32 {
        self.num_1 * self.num_2
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            sequence_index: 0,
            num_1: 0,
            num_2: 0,
        }
    }
}

pub fn day3() {
    let file_contents = fs::read_to_string(INPUT_FILE)
        .expect(format!("Could not read file {}", INPUT_FILE).as_str());

    let mut sum = 0;

    // 1-8 are reserved for states relating to mul(X, Y)
    // 9-11 is reserved to do()
    // 12-15 is reserved for don't()

    let mut enabled = true;

    let mut state = State::default();

    for (_index, ch) in file_contents.char_indices() {
        match (state.sequence_index, ch) {
            // ul(
            (1, 'u') | (2, 'l') | (3, '(') | (5, ',') => state.sequence_index += 1,
            (4, c) if c.is_digit(10) => {
                state.sequence_index += 1;
                state.num_1 = c.to_digit(10).unwrap();
            }
            (5, c) if c.is_digit(10) => state.num_1 = state.num_1 * 10 + c.to_digit(10).unwrap(),
            (6, c) if c.is_digit(10) => {
                state.sequence_index += 1;
                state.num_2 = c.to_digit(10).unwrap();
            }
            (7, c) if c.is_digit(10) => state.num_2 = state.num_2 * 10 + c.to_digit(10).unwrap(),
            (7, ')') => {
                debug!("trying to add mul({},{})", state.num_1, state.num_2);
                if enabled {
                    sum += state.evaluate();
                    debug!("sum increased to {}", sum);
                }
                state = State::default();
            }
            // o(
            (9, 'o') | (10, '(') => state.sequence_index += 1,
            (11, ')') => {
                debug!("enabled set to true");
                enabled = true;
                state = State::default();
            }
            // n't(
            (10, 'n') => state.sequence_index = 12,
            (12, '\'') | (13, 't') | (14, '(') => state.sequence_index += 1,
            (15, ')') => {
                debug!("enabled set to {:?}", false);
                enabled = false;
                state = State::default();
            }
            (_x, 'm') => state.sequence_index = 1,
            (_x, 'd') => state.sequence_index = 9,
            _ => state = State::default(),
        }
    }

    info!("Total Sum: {}", sum);
}
