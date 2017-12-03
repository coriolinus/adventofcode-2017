//! Spiral Memory
//!
//! Each square on the grid is allocated in a spiral pattern starting at a location
//! marked 1 and then counting up while spiraling outward. For example, the first
//! few squares are allocated like this:
//!
//! ```text
//! 17  16  15  14  13
//! 18   5   4   3  12
//! 19   6   1   2  11
//! 20   7   8   9  10
//! 21  22  23---> ...
//! ```
//!
//! While this is very space-efficient (no squares are skipped), requested data must
//! be carried back to square 1 (the location of the only access port for this memory
//! system) by programs that can only move up, down, left, or right. They always take
//! the shortest path: the Manhattan Distance between the location of the data and
//! square 1.

use std::cmp::{max, min};

struct SpiralMemoryRings {
    ring_maxes: Vec<usize>,
}

impl SpiralMemoryRings {
    fn new() -> SpiralMemoryRings {
        SpiralMemoryRings { ring_maxes: vec![1] }
    }

    /// Compute the max value in any given ring
    ///
    /// This value is located directly southeast of the origin at all times.
    fn ring_max(&mut self, ring: usize) -> usize {
        if ring < self.ring_maxes.len() {
            self.ring_maxes[ring]
        } else {
            while ring >= self.ring_maxes.len() {
                let pseudo_ring = self.ring_maxes.len();
                let next_value = (pseudo_ring * 8) + self.ring_maxes[pseudo_ring - 1];
                self.ring_maxes.push(next_value);
            }
            self.ring_maxes[ring]
        }
    }

    /// Compute the min value in any given ring
    ///
    /// This value is located one y-step directly north of the southest-most point
    /// in the ring for all rings > 0
    fn ring_min(&mut self, ring: usize) -> usize {
        if ring == 0 {
            1
        } else {
            1 + self.ring_max(ring - 1)
        }
    }

    /// Compute the cardinal values of the given ring
    ///
    /// These are the values due east, north, west, and south on that ring,
    /// in increasing order.
    fn cardinal(&mut self, ring: usize) -> [usize; 4] {
        if ring == 0 {
            return [1; 4];
        }
        let mut values = [0; 4];
        // compute the east value
        values[0] = self.ring_min(ring) + (ring - 1);
        // compute the other ones
        for idx in 1..4 {
            values[idx] = values[idx - 1] + (2 * ring)
        }
        assert!(!values.iter().any(|&v| v == 0));
        values
    }

    /// Find the ring on which a given value lies
    fn ring_of(&mut self, val: usize) -> usize {
        if val == 0 {
            panic!("`val 0` doesn't exist!")
        }
        let mut ring = 0;
        while val > self.ring_max(ring) {
            ring += 1;
        }
        ring
    }
}

/// Compute the number of manhattan-distance steps to the origin from a given value
///
/// This must always be equal to the ring it's on plus the minimum distance to the nearest
/// cardinal direction
pub fn steps_to_origin(val: usize) -> usize {
    let mut smr = SpiralMemoryRings::new();
    let ring = smr.ring_of(val);
    // we know that there are four items in the list, so it's safe to say that
    // at least one of them will be minimal
    ring +
        smr.cardinal(ring)
            .iter()
            .map(|&dv| max(dv, val) - min(dv, val))
            .min()
            .unwrap()
}

#[derive(Debug)]
pub struct SpiralMemory {
    origin: (usize, usize),
    memory: Vec<Vec<Option<usize>>>,
}

impl SpiralMemory {
    fn new(rings: usize) -> SpiralMemory {
        let breadth = 1 + (2 * rings);
        let mut memory = vec![vec![None; breadth]; breadth];
        let origin = (rings, rings);
        memory[origin.1][origin.0] = Some(1);
        SpiralMemory {
            origin: origin,
            memory: memory,
        }
    }
}

pub struct StressTest {
    pub spiral_memory: SpiralMemory,
    cursor: (usize, usize), // x, y
    value: usize,
}

impl StressTest {
    pub fn new(val: usize) -> StressTest {
        let mut smr = SpiralMemoryRings::new();
        let rings = smr.ring_of(val);
        let memory = SpiralMemory::new(rings);
        StressTest {
            cursor: memory.origin,
            spiral_memory: memory,
            value: val,
        }
    }

    /// Update self.point
    fn increment_cursor(&mut self) {
        let point = self.cursor;
        self.cursor = if point == self.spiral_memory.origin {
            (self.spiral_memory.origin.0 + 1, self.spiral_memory.origin.1)
        } else {
            if self.spiral_memory.memory[point.1][point.0 - 1].is_some() &&
                !self.spiral_memory.memory[point.1 - 1][point.0].is_some()
            {
                // there's something west of us and nothing north, go north
                (point.0, point.1 - 1)
            } else if self.spiral_memory.memory[point.1 + 1][point.0].is_some() &&
                       !self.spiral_memory.memory[point.1][point.0 - 1].is_some()
            {
                // something south of us, go west
                (point.0 - 1, point.1)
            } else if self.spiral_memory.memory[point.1][point.0 + 1].is_some() &&
                       !self.spiral_memory.memory[point.1 + 1][point.0].is_some()
            {
                // something east of us, go south
                (point.0, point.1 + 1)
            } else if self.spiral_memory.memory[point.1 - 1][point.0].is_some() &&
                       !self.spiral_memory.memory[point.1][point.0 + 1].is_some()
            {
                // something north of us, go east
                (point.0 + 1, point.1)
            } else {
                unreachable!()
            }
        };
    }

    fn update_at_cursor(&mut self) -> usize {
        let mut sum = 0;
        for dy in [-1_isize, 0, 1].iter() {
            for dx in [-1_isize, 0, 1].iter() {
                if !(*dx == 0 && *dy == 0) {
                    let x = if *dx == -1 {
                        self.cursor.0 - 1
                    } else {
                        self.cursor.0 + *dx as usize
                    };
                    let y = if *dy == -1 {
                        self.cursor.1 - 1
                    } else {
                        self.cursor.1 + *dy as usize
                    };
                    if let Some(val) = self.spiral_memory.memory[y][x] {
                        sum += val;
                    }
                }
            }
        }
        self.spiral_memory.memory[self.cursor.1][self.cursor.0] = Some(sum);
        sum
    }

    fn next(&mut self) -> usize {
        self.increment_cursor();
        self.update_at_cursor()
    }

    pub fn first_cell_greater_than(&mut self) -> usize {
        let mut rv = self.next();
        while rv <= self.value {
            rv = self.next();
        }
        rv
    }
}
