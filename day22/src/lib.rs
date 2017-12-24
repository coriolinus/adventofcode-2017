use std::fmt;

#[macro_use]
extern crate util;

pub type Map = Vec<Vec<bool>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Default for Direction {
    fn default() -> Direction {
        Direction::Up
    }
}

impl Direction {
    pub fn right(&self) -> Direction {
        use Direction::*;
        match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    pub fn left(&self) -> Direction {
        use Direction::*;
        match *self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }
}

pub const MAP_FACTOR: usize = 100;

pub fn generate_map(input: &str) -> Map {
    let lines = input.lines()
        .map(|line| line.trim())
        .filter(|&line| line.len() > 0)
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("unexpected char in input"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let nr_rows = lines.len();
    let map_size = nr_rows * MAP_FACTOR + 1;

    let mut out = Vec::with_capacity(map_size);
    let nr_prefix_rows = (map_size - nr_rows) / 2;

    for _ in 0..nr_prefix_rows {
        out.push(vec![false; map_size]);
    }
    for row_idx in 0..nr_rows {
        let mut row = vec![false; map_size];
        // assume a square input
        for col_idx in 0..nr_rows {
            row[col_idx + nr_prefix_rows] = lines[row_idx][col_idx];
        }
        out.push(row);
    }
    for _ in 0..nr_prefix_rows {
        out.push(vec![false; map_size]);
    }

    out
}

pub struct Sporifica<'a> {
    row: usize,
    col: usize,
    facing: Direction,
    map: &'a mut Map,
}

impl<'a> Sporifica<'a> {
    pub fn new(map: &'a mut Map) -> Sporifica<'a> {
        let half = map.len() / 2;
        Sporifica {
            row: half,
            col: half,
            facing: Direction::Up,
            map: map,
        }
    }

    fn current(&mut self) -> &mut bool {
        &mut self.map[self.row][self.col]
    }

    pub fn burst(&mut self) -> bool {
        self.facing = if *self.current() {
            self.facing.right()
        } else {
            self.facing.left()
        };
        *self.current() = !*self.current();
        let activated = *self.current();
        {
            use Direction::*;
            match self.facing {
                Up => self.row -= 1,
                Right => self.col += 1,
                Down => self.row += 1,
                Left => self.col -= 1,
            }
        }
        activated
    }

    pub fn burst_n(&mut self, n: usize) -> usize {
        let mut count = 0;
        for i in 0..n {
            // debug_println!("bursts {}:", i);
            // debug_println!("activations: {}", count);
            // debug_println!("{}", self.to_string());
            // debug_println!("");

            if self.burst() {
                count += 1;
            }
        }
        count
    }
}

impl<'a> fmt::Display for Sporifica<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.map.len() {
            for col in 0..self.map.len() {
                if row == self.row && col == self.col {
                    write!(f, "[")?;
                } else {
                    write!(f, " ")?;
                }
                if self.map[row][col] {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
                if row == self.row && col == self.col {
                    write!(f, "]")?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "{:?}", self.facing)
    }
}
