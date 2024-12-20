use adventofcode24::week3::day19::day19;
use log::info;
use std::time::Instant;

fn main() {
    env_logger::init();
    let now = Instant::now();
    {
        day19();
    }
    let elapsed = now.elapsed();
    info!("Day 19 took: {elapsed:.2?}");
}
