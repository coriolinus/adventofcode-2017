extern crate day15;
use day15::{Generator, judge};

const A_INIT: u64 = 512;
const B_INIT: u64 = 191;

const ROUNDS: usize = 40_000_000;

fn main() {
    println!(
        "count: {}",
        judge(
            Generator::generator_a(A_INIT),
            Generator::generator_b(B_INIT),
            ROUNDS,
        )
    );
    println!(
        "mod count: {}",
        judge(
            Generator::generator_a(A_INIT).filter(|&v| v % 4 == 0),
            Generator::generator_b(B_INIT).filter(|&v| v % 8 == 0),
            5_000_000,
        )
    );
}
