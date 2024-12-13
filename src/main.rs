use adventofcode24::day12::day12;
use log::info;
use std::time::Instant;

fn main() {
    env_logger::init();
    let now = Instant::now();
    {
        day12();
    }
    let elapsed = now.elapsed();
    info!("Day 12 took: {elapsed:.2?}");
}
