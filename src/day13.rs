use std::fs;

use log::{info, trace};

use regex::Regex;

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\day13.txt";

const INPUT_PATTERN: &str = r"(?m)(?R)^Button A: X\+(?<button_a_x>\d+), Y\+(?<button_a_y>\d+)\r\nButton B: X\+(?<button_b_x>\d+), Y\+(?<button_b_y>\d+)\r\nPrize: X=(?<prize_x>\d+), Y=(?<prize_y>\d+)(?:\r\n)?$";

const A_COST: i64 = 3;
const B_COST: i64 = 1;

const CONVERSION_ERROR: i64 = 10_000_000_000_000;

#[derive(Debug)]
struct Machine {
    button_a: Coord2D,
    button_b: Coord2D,
    prize: Coord2D,
}

impl Machine {
    fn new(a_x: i64, a_y: i64, b_x: i64, b_y: i64, p_x: i64, p_y: i64) -> Self {
        Self {
            button_a: Coord2D::new(a_x, a_y),
            button_b: Coord2D::new(b_x, b_y),
            prize: Coord2D::new(p_x, p_y),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Coord2D {
    x: i64,
    y: i64,
}

impl Coord2D {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

pub fn day13() {
    let file_contents =
        fs::read_to_string(INPUT_FILE).expect(format!("Could not read file {INPUT_FILE}").as_str());

    let re = Regex::new(INPUT_PATTERN).unwrap();
    let results = re.captures_iter(&file_contents).map(|c| {
        Machine::new(
            c.name("button_a_x").unwrap().as_str().parse().unwrap(),
            c.name("button_a_y").unwrap().as_str().parse().unwrap(),
            c.name("button_b_x").unwrap().as_str().parse().unwrap(),
            c.name("button_b_y").unwrap().as_str().parse().unwrap(),
            c.name("prize_x").unwrap().as_str().parse::<i64>().unwrap() + CONVERSION_ERROR,
            c.name("prize_y").unwrap().as_str().parse::<i64>().unwrap() + CONVERSION_ERROR,
        )
    });
    let mut token_count = 0;
    for Machine {
        button_a,
        button_b,
        prize,
    } in results
    {
        trace!("Machine {{button_a:{button_a:?}, button_b: {button_b:?}, prize: {prize:?}}}");
        let det_div = button_a.x * button_b.y - button_b.x * button_a.y;
        if det_div == 0 {
            continue;
        }
        let a_numerator = prize.x * button_b.y - prize.y * button_b.x;
        let b_numerator = prize.y * button_a.x - prize.x * button_a.y;
        if a_numerator % det_div != 0 || b_numerator % det_div != 0 {
            continue;
        }
        let a = a_numerator / det_div;
        let b = b_numerator / det_div;
        if a < 0 || b < 0 {
            continue;
        }
        trace!("(A, B) = ({a}, {b})");
        token_count += A_COST * a + b * B_COST;
    }
    info!("Minimum tokens spent: {token_count}");
}
