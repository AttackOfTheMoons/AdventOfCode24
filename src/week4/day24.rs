use std::{collections::HashMap, fs, process::Output};

use log::{debug, info, trace};

use regex::Regex;
use unordered_n_tuple::{UnorderedNTuple, UnorderedPair};

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\input\\week_4\\day24.txt";
const INPUTS_PATTERN: &str = r"(?m)(?R)^(?<label>[a-z]\d\d): (?<state>1|0)$";
const GATES_PATTERN: &str = r"(?m)(?R)^(?<input1>[a-z\d]{3}) (?<type>AND|XOR|OR) (?<input2>[a-z\d]{3}) -> (?<output>[a-z\d]{3})$";

const Z_PATTERN: &str = r"z(\d\d)";

#[derive(Debug)]
enum GateType {
    AND,
    OR,
    XOR,
}

impl GateType {
    fn eval(&self, inputs: (bool, bool)) -> bool {
        match self {
            &GateType::OR => inputs.0 || inputs.1,
            &GateType::AND => inputs.0 && inputs.1,
            &GateType::XOR => inputs.0 ^ inputs.1,
        }
    }
}

#[derive(Debug)]
struct Gate {
    gate_type: GateType,
    inputs: UnorderedNTuple<String, 2>,
    output: String,
}

impl Gate {
    fn new(gate_type: &str, inputs: UnorderedNTuple<String, 2>, output: String) -> Self {
        let gate_type = match gate_type {
            "AND" => GateType::AND,
            "OR" => GateType::OR,
            "XOR" => GateType::XOR,
            _ => panic!(),
        };
        Self {
            gate_type,
            inputs,
            output,
        }
    }

    fn inputs_present(&self, inputs: &HashMap<String, bool>) -> bool {
        for input in self.inputs.0.iter() {
            if !inputs.contains_key(input) {
                return false;
            }
        }
        true
    }

    fn eval(&self, inputs: &HashMap<String, bool>) -> bool {
        let input_vals = (
            *inputs.get(&self.inputs.0[0]).unwrap(),
            *inputs.get(&self.inputs.0[1]).unwrap(),
        );
        self.gate_type.eval(input_vals)
    }
}

pub fn day24() {
    let file_contents =
        fs::read_to_string(INPUT_FILE).expect(format!("Could not read file {INPUT_FILE}").as_str());

    let mut parts = file_contents.split("\r\n\r\n");
    let re = Regex::new(INPUTS_PATTERN).unwrap();

    let mut inputs = re
        .captures_iter(&file_contents)
        .map(|capture| {
            (
                capture.name("label").unwrap().as_str().to_string(),
                if "1" == capture.name("state").unwrap().as_str() {
                    true
                } else {
                    false
                },
            )
        })
        .collect::<HashMap<String, bool>>();

    let re = Regex::new(GATES_PATTERN).unwrap();

    let mut gates = re
        .captures_iter(&file_contents)
        .map(|capture| {
            Gate::new(
                capture.name("type").unwrap().as_str(),
                UnorderedNTuple::from([
                    capture.name("input1").unwrap().as_str().to_string(),
                    capture.name("input2").unwrap().as_str().to_string(),
                ]),
                capture.name("output").unwrap().as_str().to_string(),
            )
        })
        .collect::<Vec<_>>();

    loop {
        let mut todo_gates = Vec::new();
        for gate in gates {
            if gate.inputs_present(&inputs) {
                let result = gate.eval(&inputs);
                inputs.insert(gate.output, result);
            } else {
                todo_gates.push(gate);
            }
        }
        if todo_gates.is_empty() {
            break;
        }
        gates = todo_gates;
    }

    let re = Regex::new(Z_PATTERN).unwrap();

    let mut z_vec = inputs
        .into_iter()
        .filter_map(|(key, value)| match re.captures(key.as_str()) {
            Some(key_place) => Some((
                key_place
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap_or_else(|_| {
                        panic!(
                            "Key was unable to be parsed {:?}",
                            key_place.get(1).unwrap()
                        )
                    }),
                value,
            )),
            None => None,
        })
        .collect::<Vec<_>>();

    z_vec.sort_by(|a, b| b.0.cmp(&a.0));

    debug!("z_vec: {z_vec:?}");

    let z = z_vec
        .into_iter()
        .fold(0_u64, |a, (_, b)| (a << 1) | if b { 1 } else { 0 });

    info!("z = {z}");
}
