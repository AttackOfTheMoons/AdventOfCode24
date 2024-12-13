use adventofcode24::day13::day13;
use log::info;
use std::time::Instant;

fn main() {
    env_logger::init();
    let now = Instant::now();
    {
        day13();
    }
    let elapsed = now.elapsed();
    info!("Day 13 took: {elapsed:.2?}");
}
