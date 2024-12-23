use adventofcode24::week4::day22::day22;
use log::info;
use std::time::Instant;

fn main() {
    env_logger::init();
    let now = Instant::now();
    {
        day22();
    }
    let elapsed = now.elapsed();
    info!("Day 22 took: {elapsed:.2?}");
}
