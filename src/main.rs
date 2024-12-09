use log::info;
use std::time::Instant;

use adventofcode24::day8::day8;

fn main() {
    env_logger::init();
    let now = Instant::now();
    {
        day8();
    }
    let elapsed = now.elapsed();
    info!("Day 8 took: {elapsed:.2?}");
}
