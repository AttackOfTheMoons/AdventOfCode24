use adventofcode24::week3::day18::day18;
use log::info;
use std::time::Instant;

fn main() {
    env_logger::init();
    let now = Instant::now();
    {
        day18();
    }
    let elapsed = now.elapsed();
    info!("Day 18 took: {elapsed:.2?}");
}
