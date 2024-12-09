use log::info;
use std::time::Instant;

use adventofcode24::day9::day9;

fn main() {
    env_logger::init();
    let now = Instant::now();
    {
        day9();
    }
    let elapsed = now.elapsed();
    info!("Day 9 took: {elapsed:.2?}");
}
