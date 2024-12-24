use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;
use log::{debug, info, trace};
use unordered_n_tuple::UnorderedNTuple;

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\input\\week_4\\day23.txt";

pub fn day23() {
    let file_contents =
        fs::read_to_string(INPUT_FILE).expect(format!("Could not read file {INPUT_FILE}").as_str());

    part_two(&file_contents);
}

fn part_two(file_contents: &String) {
    let mut nodes = HashSet::new();
    for pair in file_contents.split("\r\n").filter(|s| !s.is_empty()) {
        let mut pair_parts = pair.split("-");
        nodes.insert(pair_parts.next().unwrap());
        nodes.insert(pair_parts.next().unwrap());
    }

    let mut lan_parties_pairs = HashSet::new();
    for pair in file_contents.split("\r\n").filter(|s| !s.is_empty()) {
        let mut pair_parts = pair.split("-");
        let pair = (pair_parts.next().unwrap(), pair_parts.next().unwrap());
        lan_parties_pairs.insert(UnorderedNTuple::from([pair.0, pair.1]));
    }
    let mut pairs: HashMap<&str, HashSet<&str>> = HashMap::new();
    for pair in file_contents.split("\r\n").filter(|s| !s.is_empty()) {
        let mut pair_parts = pair.split("-");
        let pair = (pair_parts.next().unwrap(), pair_parts.next().unwrap());
        pairs.entry(pair.0).or_insert(HashSet::new()).insert(pair.1);
        pairs.entry(pair.1).or_insert(HashSet::new()).insert(pair.0);
    }

    let mut highest: Option<(&str, usize)> = None;
    for &node in nodes.iter() {
        let mut counted = HashSet::new();
        for &i in pairs.get(node).unwrap().iter() {
            for &j in pairs.get(node).unwrap().iter().filter(|&&s| *s != *i) {
                let neighbor_pair = UnorderedNTuple::from([i, j]);
                if lan_parties_pairs.contains(&neighbor_pair) {
                    counted.insert(neighbor_pair);
                }
            }
        }

        match highest {
            Some(high) => {
                if high.1 < counted.len() {
                    highest = Some((node, counted.len()));
                }
            }
            None => highest = Some((node, counted.len())),
        }
    }

    let (node, _) = highest.unwrap();

    let mut counted = HashSet::new();
    for i in pairs.get(node).unwrap().iter().combinations(2) {
        let neighbor_pair = UnorderedNTuple::from([&i[0], *i[1]]);
        if lan_parties_pairs.contains(&neighbor_pair) {
            counted.insert(neighbor_pair);
        }
    }

    let mut nodes = HashSet::<&str>::from_iter(counted.iter().flat_map(|&tup| tup.0))
        .iter()
        .map(|&s| s)
        .collect_vec();
    nodes.push(node);
    nodes.sort();
    info!("The password is '{}'", nodes.join(","));
}

#[allow(dead_code)]
fn part_one(file_contents: &String) {
    let mut pairs: HashMap<&str, HashSet<&str>> = HashMap::new();
    for pair in file_contents.split("\r\n").filter(|s| !s.is_empty()) {
        let mut pair_parts = pair.split("-");
        let pair = (pair_parts.next().unwrap(), pair_parts.next().unwrap());
        pairs.entry(pair.0).or_insert(HashSet::new()).insert(pair.1);
        pairs.entry(pair.1).or_insert(HashSet::new()).insert(pair.0);
    }

    let mut lan_party_3 = HashSet::new();
    for (&key, links) in pairs.iter() {
        let neighbors = Vec::from_iter(links);
        for &&i in neighbors.iter() {
            for &&j in neighbors.iter().filter(|&&&j| *j != *i) {
                if let Some(js_links) = pairs.get(j) {
                    if js_links.contains(&i)
                        && (i.starts_with("t") || j.starts_with("t") || key.starts_with("t"))
                    {
                        lan_party_3.insert(UnorderedNTuple::from([i, j, key]));
                    }
                }
            }
        }
    }
    // part 1
    info!("There were {} 3 people lan parties", lan_party_3.len());
}
