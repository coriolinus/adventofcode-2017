extern crate util;
use util::read_file;

extern crate day20;
use day20::{parse_particle, simulate};

fn main() {
    let input = read_file("input.txt");
    let mut particles = input.trim()
        .lines()
        .map(|line| parse_particle(line.trim()).expect("failed to parse particle"))
        .collect::<Vec<_>>();
    let min_motion_idx = particles.iter()
        .enumerate()
        .map(|(idx, p)| (p.get_acc().abs_sum(), p.get_vel().abs_sum(), p.get_pos().abs_sum(), idx))
        .min()
        .map(|(_, _, _, idx)| idx)
        .expect("no min motion particle?");

    println!(
        "min motion particle: {} ({:?})",
        min_motion_idx,
        particles[min_motion_idx],
    );

    println!("simulating...");
    simulate(&mut particles);
    println!("remaining particles: {}", particles.len());
}
