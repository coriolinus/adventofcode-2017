extern crate day24;
use day24::{max_length_strength_bridge, max_strength_bridge, Component};

extern crate util;
use util::file_as_by;

fn main() {
    let input = file_as_by::<usize, _>("input.txt", |line| line.split("/").collect())
        .expect("problem reading input");
    let components = input
        .iter()
        .map(|c| Component::new(c[0], c[1]))
        .collect::<Vec<_>>();

    println!(
        "max strength bridge: {:?}",
        max_strength_bridge(&components).map(|b| b.strength())
    );

    println!(
        "max length bridge strength: {:?}",
        max_length_strength_bridge(&components).map(|b| b.strength())
    );
}
