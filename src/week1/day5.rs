use log::{debug, info, trace};
use std::time::Instant;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\day5.txt";

pub fn day5() {
    let file_contents =
        fs::read_to_string(INPUT_FILE).expect(format!("Could not read file {INPUT_FILE}").as_str());

    let (rules, orders) = parse_order_and_rules(file_contents);

    let (rules_copy, orders_copy) = (rules.clone(), orders.clone());

    let naive_sum: i32;

    let now = Instant::now();
    {
        naive_sum = naive_sort(rules_copy, orders_copy);
    }
    let elapsed = now.elapsed();
    info!("Naive Solution Sum: {naive_sum}. Time elapsed: {elapsed:.2?}");
    // let now = Instant::now();
    // {
    //     efficient_sum = efficient_sort(rules, orders);
    // }
    // let elapsed = now.elapsed();
    // info!("Efficient Solution Sum: {efficient_sum}. Time elapsed: {elapsed:.2?}");
}

// This makes the assumption that the rules are complete (ie. every rule possible to exist exists)
// This isn't more efficient because if the rules are complete, then there are going to be a ton of rules to process per order
// if there was only 1 order, I assert this is faster than the other solution.
#[allow(dead_code)]
fn efficient_sort(rules: Vec<(i32, i32)>, orders: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for order in orders {
        let order_set: HashSet<i32> = HashSet::from_iter(order.iter().cloned());

        let mut rule_map = HashMap::new();

        // rule map contains (item, number of times that rule is present).
        for rule in rules.iter() {
            if order_set.contains(&rule.0) && order_set.contains(&rule.1) {
                let count: &mut i32 = rule_map.entry(rule.0).or_insert(0);
                *count += 1;
            }
        }

        // now sort them based on their rule map value.
        let mut count_vec: Vec<_> = rule_map.iter().collect();
        count_vec.sort_by(|a, b| b.1.cmp(a.1));

        let sorted_order: Vec<_> = count_vec.iter().map(|&(k, _v)| *k).collect();

        // the last item will be missing
        debug!("Sorted order: {sorted_order:?}");

        let mut invalid = false;
        for (index, &num) in order.iter().enumerate() {
            if index == sorted_order.len() {
                if rule_map.contains_key(&num) {
                    invalid = true;
                }
            } else {
                match sorted_order.get(index) {
                    Some(&x) => {
                        if x != num {
                            invalid = true;
                        }
                    }
                    None => invalid = true,
                }
            }
        }

        if !invalid {
            sum += order.get(order.len() / 2).unwrap();
        }
    }
    return sum;
}

// This makes no assumptions about the rules, however a "correct" order implies all numbers have a rule associated with them.
#[allow(dead_code)]
fn naive_sort(rules: Vec<(i32, i32)>, orders: Vec<Vec<i32>>) -> i32 {
    // This map stores which numbers show in which lists. ie. 1|2 would make 2: [1] since 2 appears behind 1.
    let mut secondary_map: HashMap<i32, HashSet<i32>> = HashMap::new();

    for (first_num, second_num) in rules.iter() {
        match secondary_map.get_mut(second_num) {
            Some(set) => {
                set.insert(*first_num);
            }
            None => {
                let mut new_set = HashSet::new();
                new_set.insert(*first_num);
                secondary_map.insert(*second_num, new_set);
            }
        }
    }

    debug!("secondary_map: {secondary_map:?}");

    let mut sum = 0;

    for mut order in orders {
        let order_len = order.len();
        let mut had_issue = false;
        'inner_loop: for index in 0..order_len {
            loop {
                let number = order[index];
                trace!("Looking at {number}");
                match secondary_map.get(&number) {
                    Some(set) => {
                        for i in (index + 1)..order_len {
                            if set.contains(order.get(i).unwrap()) {
                                (order[i], order[index]) = (order[index], order[i]);
                                had_issue = true;
                                continue;
                            }
                        }
                        break;
                    }
                    None => {
                        continue 'inner_loop;
                    }
                }
            }
        }
        debug!("Sorted order: {:?}", order);
        if !had_issue {
            sum += order.get(order_len / 2).unwrap();
        }
    }
    return sum;
}

fn parse_order_and_rules(raw_string: String) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut parts = raw_string.split("\r\n\r\n");
    let rules = parts
        .next()
        .expect("Could not find rules")
        .split("\r\n")
        .map(|s| {
            let mut nums = s.split("|");
            (
                nums.next()
                    .unwrap_or_else(|| panic!("Couldn't find first num in rule {s}"))
                    .parse::<i32>()
                    .unwrap_or_else(|_| panic!("Couldn't parse first num {s} as i32")),
                nums.next()
                    .unwrap_or_else(|| panic!("Couldn't find second num in rule {s}"))
                    .parse::<i32>()
                    .unwrap_or_else(|_| panic!("Couldn't parse second num {s} as i32")),
            )
        })
        .collect::<Vec<_>>();
    let orders = parts
        .next()
        .unwrap_or_else(|| {
            panic!("Could not find orders");
        })
        .split("\r\n")
        .map(|s| {
            s.split(",")
                .map(|x| {
                    x.parse::<i32>()
                        .unwrap_or_else(|_| panic!("Failed to parse {x} as i32 from: {s}"))
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (rules, orders)
}
