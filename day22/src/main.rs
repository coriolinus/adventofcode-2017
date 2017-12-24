extern crate day22;
use day22::{generate_map, Sporifica};

extern crate util;
use util::read_file;

fn main() {
    let input = read_file("input.txt");
    let mut map = generate_map(&input);

    let mut virus = Sporifica::new(&mut map);
    let bursts = 10_000_000;
    println!("activating in {}: {}", bursts, virus.burst_n(bursts));
}
