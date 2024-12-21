use log::{info, trace};
use regex::Regex;
use std::fs;

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\input\\week_3\\day17.txt";

const INPUT_PATTERN: &str = r"(?m)(?R)^Register A: (?<a_val>\d+)\r\nRegister B: (?<b_val>\d+)\r\nRegister C: (?<c_val>\d+)\r\n\r\nProgram: (?<instructions>[0-7,]+)$";

// const GUESS: u64 = 0b111_111_111_111_111_111_111_000_000_000_000_000_000_000_000_011;

pub fn day17() {
    let file_contents =
        fs::read_to_string(INPUT_FILE).expect(format!("Could not read file {INPUT_FILE}").as_str());

    let re = Regex::new(INPUT_PATTERN).unwrap();
    let c = re.captures(&file_contents).unwrap();
    // part 2 says the computer has been corrupted!
    let mut _computer = Computer::new(
        c.name("a_val").unwrap().as_str().parse().unwrap(),
        c.name("b_val").unwrap().as_str().parse().unwrap(),
        c.name("c_val").unwrap().as_str().parse().unwrap(),
    );

    let instructions = c.name("instructions").unwrap().as_str();

    let instructions_vec = instructions
        .split(",")
        .map(|c| c.parse().unwrap())
        .collect::<Vec<u64>>();

    // 2 : (3, 6)
    let guess_vec = vec![1, 0, 3, 5, 5, 1, 0, 4, 3, 2, 5, 3, 6, 7, 6, 4];

    let place = 15;

    for i in 0_u64..8 {
        let mut guess_copy = guess_vec.clone();
        *guess_copy.get_mut(place).unwrap() = i;
        let guess: u64 = guess_copy
            .into_iter()
            .reduce(|r1, r2| (r1 << 3 | r2))
            .unwrap();
        let mut computer = Computer::new(guess, 0, 0);
        computer.run_str(instructions.to_string());
        let mut valid = true;
        for c in (15 - place)..15 {
            if computer.out.get(c).unwrap() != instructions_vec.get(c).unwrap() {
                valid = false
            }
        }
        if valid {
            trace!("Real is {:?}", instructions_vec);
            trace!("Out was {:?}", computer.out);
            trace!("i = {i}");
            trace!("{guess}");
        }
    }
}

#[allow(dead_code)]
fn part_one(mut computer: Computer, instructions: Vec<u64>) {
    computer.run(instructions);

    info!("Output: {}", computer.read_out())
}

pub struct Computer {
    pub a_val: u64,
    pub b_val: u64,
    pub c_val: u64,
    pub out: Vec<u64>,
    instruction_ptr: usize,
}

impl Computer {
    pub fn new(a_val: u64, b_val: u64, c_val: u64) -> Self {
        Self {
            a_val,
            b_val,
            c_val,
            out: Default::default(),
            instruction_ptr: 0,
        }
    }

    pub fn run_str(&mut self, instructions: String) {
        let instructions_vec = instructions
            .split(",")
            .map(|c| c.parse().unwrap())
            .collect::<Vec<u64>>();

        self.run(instructions_vec);
    }

    pub fn run(&mut self, instructions: Vec<u64>) {
        while self.instruction_ptr < instructions.len() {
            let opcode = instructions.get(self.instruction_ptr);
            let operand = instructions.get(self.instruction_ptr + 1);
            if opcode.is_none() || operand.is_none() {
                break;
            }

            self.compute(*opcode.unwrap(), *operand.unwrap());
        }
    }

    pub fn read_out(&self) -> String {
        self.out
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn combo(&self, operand: u64) -> u64 {
        match operand {
            0..=3 => operand,
            4 => self.a_val,
            5 => self.b_val,
            6 => self.c_val,
            7 => todo!("7 combo() operand is reserved and will not appear in valid programs"),
            other => panic!("Unrecognized operand '{other}' passed to combo()"),
        }
    }

    pub fn compute(&mut self, opcode: u64, operand: u64) {
        match opcode {
            0 => {
                // `adv`
                self.a_val = self.a_val / 2_u64.pow(self.combo(operand).try_into().unwrap());
            }
            1 => {
                // `bxl`
                self.b_val ^= operand;
            }
            2 => {
                // `bst`
                self.b_val = self.combo(operand) % 8;
            }
            3 => {
                // `jnz`
                if self.a_val != 0 {
                    self.instruction_ptr = operand.try_into().unwrap();
                    return;
                }
            }
            4 => {
                // `bxc`
                self.b_val ^= self.c_val;
            }
            5 => {
                // `out`
                self.out.push(self.combo(operand) % 8);
            }
            6 => {
                // `bdv`
                self.b_val = self.a_val / 2_u64.pow(self.combo(operand).try_into().unwrap());
            }
            7 => {
                // `cdv`
                self.c_val = self.a_val / 2_u64.pow(self.combo(operand).try_into().unwrap());
            }
            other => panic!("Unrecognized opcode '{other}' passed to compute()"),
        }
        self.instruction_ptr += 2;
    }
}
