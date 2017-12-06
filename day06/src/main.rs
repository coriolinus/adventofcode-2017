extern crate day06;
use day06::redistributions_cycle_len;

extern crate util;
use util::file_as;

fn main() {
    for line in file_as::<usize>("input.txt").expect("Problem reading input.txt") {
        let (total_cycles, loop_length) = redistributions_cycle_len(&line);
        println!("Total cycles: {}", total_cycles);
        println!("Loop length: {}", loop_length);
    }
}
