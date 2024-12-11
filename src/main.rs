use adventofcode24::day11::day11;
use log::info;
use std::time::Instant;

fn main() {
    env_logger::init();
    let now = Instant::now();
    {
        day11();
    }
    let elapsed = now.elapsed();
    info!("Day 11 took: {elapsed:.2?}");
}
