use adventofcode24::week4::{day25::day25};
use log::info;
use std::time::Instant;

fn main() {
    env_logger::init();
    let now = Instant::now();
    {
        day25();
    }
    let elapsed = now.elapsed();
    info!("Day 25 took: {elapsed:.2?}");
}
