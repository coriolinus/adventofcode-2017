use std::collections::HashSet;
use std::ops::{Add, AddAssign};

pub mod parser;
pub use parser::parse_particle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector3 {
    pub fn new(x: i64, y: i64, z: i64) -> Vector3 {
        Vector3 { x: x, y: y, z: z }
    }

    pub fn abs_sum(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Particle {
    position: Vector3,
    velocity: Vector3,
    acceleration: Vector3,
}

impl Particle {
    pub fn new(pos: Vector3, vel: Vector3, acc: Vector3) -> Particle {
        Particle {
            position: pos,
            velocity: vel,
            acceleration: acc,
        }
    }

    /// Update this particle's velocity and position, and
    /// return its manhattan distance from the origin.
    pub fn tick(&mut self) -> i64 {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.position.abs_sum()
    }

    pub fn get_pos(&self) -> &Vector3 {
        &self.position
    }

    pub fn get_acc(&self) -> &Vector3 {
        &self.acceleration
    }

    pub fn get_vel(&self) -> &Vector3 {
        &self.velocity
    }
}

/// update the simulation state in O(n**2)
///
/// not great, but should be sufficient, and I'm
/// too tired to think of something better right now.
pub fn destroy_tick(particles: &mut Vec<Particle>) {
    let mut destroyed = HashSet::new();

    for (outer_idx, outer_particle) in particles.iter().enumerate() {
        for (inner_idx, inner_particle) in particles.iter().enumerate() {
            if outer_idx != inner_idx && outer_particle.position == inner_particle.position {
                destroyed.insert(outer_particle.clone());
                destroyed.insert(inner_particle.clone());
            }
        }
    }

    particles.retain(|particle| !destroyed.contains(particle));

    for particle in particles.iter_mut() {
        particle.tick();
    }
}

/// run the simulation until 100 ticks with no destruction
pub fn simulate(particles: &mut Vec<Particle>) {
    let mut tick = 0;
    let mut old_plen = particles.len();
    while tick < 100 {
        destroy_tick(particles);
        if old_plen == particles.len() {
            tick += 1;
        } else {
            tick = 0;
        }
        old_plen = particles.len()
    }
}
