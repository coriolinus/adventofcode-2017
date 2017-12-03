extern crate day03;
use day03::{steps_to_origin, StressTest};

fn main() {
    let val = 277678;
    println!("{} takes {} steps to the origin", val, steps_to_origin(val));
    println!(
        "Stress test value: {}",
        StressTest::new(val).first_cell_greater_than()
    );
}
