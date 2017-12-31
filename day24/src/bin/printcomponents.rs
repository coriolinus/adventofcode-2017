extern crate day24;
use day24::{ComponentBin, Component};

extern crate util;
use util::file_as_by;

fn main() {
    let input = file_as_by::<usize, _>("input.txt", |line| line.split("/").collect())
        .expect("problem reading input");
    let components = input.iter().map(|c| Component::new(c[0], c[1])).collect::<Vec<_>>();

    let mut bin = ComponentBin::new();
    for component in components.iter() {
        bin.insert(*component);
    }
    for (key, cs) in bin.iter() {
        println!("{}:", key);
        for c in cs.iter() {
            println!("  {} ({})", c.other(*key).expect("bad key"), c);
        }
    }
}
