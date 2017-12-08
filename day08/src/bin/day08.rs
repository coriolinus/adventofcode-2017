extern crate day08;
use day08::{execute, parse, max_value_in, execute_collecting_max};

extern crate util;
use util::read_file;

fn main() {
    let input = read_file("input.txt");
    let program = parse(&input);
    let registers = execute(&program);

    println!("Max value in any register: {:?}", max_value_in(&registers));
    println!(
        "Max value ever to appear: {:?}",
        execute_collecting_max(&program)
    );
}
