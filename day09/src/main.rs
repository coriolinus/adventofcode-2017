extern crate day09;
use day09::parse;

extern crate util;
use util::read_file;

fn main() {
    let input = read_file("input.txt");
    let (ast, _) = parse(&input).expect("Couldn't parse input");
    let group = ast.ok().expect("Outer parse was garbage not group");

    println!("total score: {:?}", group.score());
    println!("total garbage chars: {}", group.garbage_chars());
}
