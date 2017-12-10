extern crate day10;
use day10::{LoopHash, hash};

extern crate util;
use util::{file_as_by, read_file};

fn main() {
    for hash_chain in file_as_by::<usize, _>("input.txt", |line| {
        line.split(',')
            .filter(|ref token| !token.is_empty())
            .collect()
    }).expect("Problem parsing input")
    {
        println!("Computing hash using: {:?}", hash_chain);
        let mut lh = LoopHash::new();
        lh.twist_list(&hash_chain);
        println!("Initial product: {}", lh.initial_product());
    }

    // part 2
    let input_str = read_file("input.txt");
    println!("Hash of input file: {}", hash(&input_str.trim().as_bytes()))
}
