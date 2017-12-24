extern crate day21;
use day21::{base, parse_rule, generate_rules, enhance, Image};

extern crate util;
use util::read_file;

fn count_on_bits(image: &Image) -> usize {
    image.iter()
        .map(|row| row.iter().filter(|&&pixel| pixel).count())
        .sum()
}

fn main() {
    let input = read_file("input.txt");
    let given_rules: Vec<_> = input.lines()
        .map(|line| parse_rule(line.trim()).expect("all rules must be parseable"))
        .collect();
    let rules = generate_rules(&*given_rules);

    let mut image = base();
    for _ in 0..18 {
        image = enhance(&rules, &image);
    }

    println!("on bits: {}", count_on_bits(&image));
}
