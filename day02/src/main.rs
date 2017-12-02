extern crate util;
use util::file_as;

extern crate day02;
use day02::{checksum, divisible_checksum};

fn main() {
    let inputs = file_as::<u64>("input.txt").expect("Couldn't parse input");
    println!("Checksum of input: {}", checksum(&inputs));
    println!(
        "Divisible checksum of input: {}",
        divisible_checksum(&inputs)
    );
}
