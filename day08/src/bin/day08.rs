extern crate day08;
use day08::parse_instruction;

extern crate util;
use util::read_file;

fn main() {
    let input = read_file("input.txt");
    for (line_no, line) in input.trim().lines().enumerate() {
        if let Err(e) = parse_instruction(line) {
            println!("Failed to parse line {} (\"{}\"):", line_no, line);
            println!("  {:?}", e);
        }
    }
}
