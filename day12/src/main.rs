extern crate day12;
use day12::{parse_connections, connected_to_zero, count_groups};

extern crate util;
use util::read_file;

fn main() {
    let input = read_file("input.txt");
    let connections = input
        .trim()
        .lines()
        .map(|line| parse_connections(line).expect("Parse error"))
        .collect::<Vec<_>>();
    let zero_graph = connected_to_zero(&connections);
    println!("# nodes connected to 0: {}", zero_graph.len());
    println!("# groups: {}", count_groups(&connections));
}
