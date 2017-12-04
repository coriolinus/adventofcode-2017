extern crate day04;
use day04::{count_unique, count_valid};

extern crate util;
use util::{read_file, tokenize};

fn main() {
    let input_str = read_file("input.txt");
    let lines = tokenize(&input_str);
    println!("unique lines in input.txt: {}", count_unique(&lines));
    println!("valid lines in input.txt: {}", count_valid(&lines));
}
