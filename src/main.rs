use adventofcode24::week3::day17::day17;
use log::info;
use std::time::Instant;

fn main() {
    env_logger::init();
    let now = Instant::now();
    {
        day17();
    }
    let elapsed = now.elapsed();
    info!("Day 17 took: {elapsed:.2?}");
}
