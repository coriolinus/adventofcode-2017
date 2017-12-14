const TEST_INPUT: &'static str = "flqrgnkx";
const INPUT: &'static str = "uugsqrei";

extern crate day14;
use day14::{count_bits_of_hashes_for, bitvec_true_count, region_count};

fn main() {
    println!(
        "test input bits set: {}",
        count_bits_of_hashes_for(TEST_INPUT)
    );
    println!("input bits set: {}", count_bits_of_hashes_for(INPUT));
    println!("input bits set (bitvec): {}", bitvec_true_count(INPUT));
    println!("test input region count: {}", region_count(TEST_INPUT));
    println!("region count: {}", region_count(INPUT));
}
