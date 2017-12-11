use std::cmp::max;

extern crate day11;
use day11::{HexPosition, HexDirection};

extern crate util;
use util::file_as_by;

fn main() {
    for directions in file_as_by::<HexDirection, _>("input.txt", |line| {
        line.split(',')
            .filter(|ref token| !token.is_empty())
            .collect()
    }).expect("Problem parsing input")
    {
        let position: HexPosition = directions.iter().sum();
        println!("Dist to origin: {}", position.min_steps_to_origin());

        let mut position = HexPosition::new();
        let mut max_dist = 0;
        for direction in directions.iter() {
            position = position + direction;
            max_dist = max(max_dist, position.min_steps_to_origin());
        }
        println!("Max dist to origin: {}", max_dist);
    }
}
