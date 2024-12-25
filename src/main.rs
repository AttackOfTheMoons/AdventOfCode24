use adventofcode24::week4::day24::day24;
use log::info;
use std::time::Instant;

fn main() {
    env_logger::init();
    let now = Instant::now();
    {
        day24();
    }
    let elapsed = now.elapsed();
    info!("Day 24 took: {elapsed:.2?}");
}
