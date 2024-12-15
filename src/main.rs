use adventofcode24::week3::day15::day15;
use log::info;
use std::time::Instant;

fn main() {
    env_logger::init();
    let now = Instant::now();
    {
        day15();
    }
    let elapsed = now.elapsed();
    info!("Day 15 took: {elapsed:.2?}");
}
