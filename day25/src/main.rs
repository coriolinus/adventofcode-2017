extern crate day25;
use day25::TuringMachine;
use day25::input::{states, CHECKSUM_AFTER, FIRST_STATE};

fn main() {
    let mut machine = TuringMachine::new(FIRST_STATE, states());
    for _ in 0..CHECKSUM_AFTER {
        machine.execute();
    }
    println!(
        "Checksum after {} steps: {}",
        CHECKSUM_AFTER,
        machine.checksum()
    );
}
