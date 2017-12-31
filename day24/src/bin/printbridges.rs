extern crate day24;
use day24::{BridgeBuilder, Component};

extern crate util;
use util::file_as_by;

fn main() {
    let input = file_as_by::<usize, _>("input.txt", |line| line.split("/").collect())
        .expect("problem reading input");
    let components = input.iter().map(|c| Component::new(c[0], c[1])).collect::<Vec<_>>();

    for bridge in BridgeBuilder::new(&components) {
        println!("{}", bridge);
    }
}
