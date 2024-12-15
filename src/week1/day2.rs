use log::{debug, info, warn};
use std::fs;

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\input\\week_1\\day2.txt";

pub fn day2() {
    let file_contents = fs::read_to_string(INPUT_FILE)
        .expect(format!("Could not read file {}", INPUT_FILE).as_str());

    info!("Safe report count: {}", count_safe_reports(file_contents));
}

fn count_safe_reports(raw_str: String) -> i32 {
    let mut sum = 0;

    'outerloop: for report in raw_str.split("\n").filter(|s| !s.is_empty()) {
        let report: Vec<i32> = report
            .trim()
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().expect("Failed while parsing a report"))
            .collect();

        for (index, _) in report.clone().into_iter().enumerate() {
            let mut r = report.clone();
            r.remove(index);
            if is_report_safe(r) {
                debug!("{:?} was marked as safe", report);
                sum += 1;
                continue 'outerloop;
            } else {
                debug!("{:?} was marked as unsafe", report);
            }
        }
    }

    return sum;
}

fn is_report_safe(report: Vec<i32>) -> bool {
    if report.len() <= 1 {
        warn!("Report length was short ({})", report.len());
        return true;
    }

    let mut y = report.iter();

    // We already checked the length so this is safe.
    let first_item = y.next().unwrap();
    let second_item = y.next().unwrap();

    let diff = second_item - first_item;

    let valid_diff = |d: i32| match d.abs() {
        0 => false,
        n if n > 3 => false,
        _ => true,
    };

    if !valid_diff(diff) {
        return false;
    }

    let increasing = diff > 0;

    let mut previous = *second_item;

    while let Some(item) = y.next() {
        let diff = item - previous;
        if !valid_diff(diff) || (increasing && diff < 0) || (!increasing && diff > 0) {
            return false;
        }
        previous = *item;
    }

    return true;
}
