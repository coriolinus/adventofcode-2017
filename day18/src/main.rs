extern crate day18;
use day18::Machine;
use day18::parser::parse_instruction;

extern crate util;
use util::read_file;

fn main() {
    let raw_input = read_file("input.txt");
    let instructions = raw_input
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .map(|line| {
            parse_instruction(line).expect("problem parsing line")
        })
        .collect::<Vec<_>>();
    let mut machine = Machine::new(&instructions);
    println!("first sound recovered: {:?}", machine.run());
}
