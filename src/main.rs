use adventofcode24::week4::day23::day23;
use log::info;
use std::time::Instant;

fn main() {
    env_logger::init();
    let now = Instant::now();
    {
        day23();
    }
    let elapsed = now.elapsed();
    info!("Day 23 took: {elapsed:.2?}");
}
