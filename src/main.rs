use adventofcode24::week3::day21::day21;
use log::info;
use std::time::Instant;

fn main() {
    env_logger::init();
    let now = Instant::now();
    {
        day21();
    }
    let elapsed = now.elapsed();
    info!("Day 21 took: {elapsed:.2?}");
}
