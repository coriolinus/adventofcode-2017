extern crate day05;
use day05::JumpMemory;

extern crate util;
use util::{file_as, flatten};

fn main() {
    let input = flatten(file_as::<isize>("input.txt").expect(
        "Error reading input.txt",
    ));
    let mut jm = JumpMemory::new(&input);
    println!("Jumps to escape: {}", jm.run());
    let mut jm2 = JumpMemory::new(&input);
    println!("2-variant jumps to escape: {}", jm2.run2());
}
