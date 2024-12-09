use std::{collections::HashMap, fs};

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\day1.txt";

pub fn day1() {
    let file_contents = fs::read_to_string(INPUT_FILE)
        .expect(format!("Could not read file {}", INPUT_FILE).as_str());

    let (left_list, right_list) = parse_input_file(file_contents);

    println!(
        "Total Distance: {}",
        total_distance(left_list.clone(), right_list.clone())
    );
    println!(
        "Similarity Score: {}",
        similarity_score(left_list, right_list)
    );
}

fn total_distance(left_list: Vec<i32>, right_list: Vec<i32>) -> i32 {
    if left_list.len() != right_list.len() {
        eprint!("Parsed list lengths are mismatched.");
    }

    let mut list1 = left_list.clone();
    list1.sort();
    let mut list2 = right_list.clone();
    list2.sort();

    let mut sum = 0;

    for i in 0..list1.len() {
        sum += (list1[i] - list2[i]).abs();
    }

    return sum;
}
fn parse_input_file(raw_string: String) -> (Vec<i32>, Vec<i32>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for (index, line) in raw_string
        .split("\n")
        .filter(|l| !l.is_empty())
        .into_iter()
        .enumerate()
    {
        let mut parts = line.trim().split(" ").filter(|s| !s.is_empty());

        let mut get_item = || {
            let str_item = parts
                .next()
                .expect(format!("Error parsing list at line {}", index + 1).as_str());

            str_item
                .parse::<i32>()
                .expect(format!("Could not parse {} to i32", str_item).as_str())
        };

        list1.push(get_item());
        list2.push(get_item());
    }

    return (list1, list2);
}

fn similarity_score(left_list: Vec<i32>, right_list: Vec<i32>) -> i32 {
    let mut right_list_map: HashMap<i32, i32> = HashMap::new();

    let mut sum = 0;

    for i in right_list {
        *right_list_map.entry(i).or_insert(0) += 1;
    }

    for i in left_list {
        sum += right_list_map.get(&i).unwrap_or(&0) * i;
    }

    return sum;
}
