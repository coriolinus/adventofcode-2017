extern crate day18;
use day18::Duet;
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
    let mut duet = Duet::new(&instructions);
    println!("Machine 1 send count: {}", duet.run());
}
