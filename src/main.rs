use adventofcode24::week3::day16::day16;
use log::info;
use std::time::Instant;

fn main() {
    env_logger::init();
    let now = Instant::now();
    {
        day16();
    }
    let elapsed = now.elapsed();
    info!("Day 16 took: {elapsed:.2?}");
}
