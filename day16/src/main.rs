extern crate day16;
use day16::{Instruction, dance, dance_repeat};

extern crate util;
use util::file_as_by;

fn main() {
    for instructions in file_as_by::<Instruction, _>(
        "input.txt",
        |line| line.split(',').collect(),
    ).expect("problem reading input")
    {
        println!("danced into arrangement: {}", dance(&instructions));
        println!(
            "dance a billion: {}",
            dance_repeat(&instructions, 1_000_000_000)
        );
    }
}
