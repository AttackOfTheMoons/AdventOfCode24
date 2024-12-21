use adventofcode24::week3::day20::day20;
use log::info;
use std::time::Instant;

fn main() {
    env_logger::init();
    let now = Instant::now();
    {
        day20();
    }
    let elapsed = now.elapsed();
    info!("Day 20 took: {elapsed:.2?}");
}
