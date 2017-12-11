use std::ascii::AsciiExt;
use std::iter::Sum;
use std::ops::Add;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HexDirection {
    N,
    Ne,
    Nw,
    S,
    Se,
    Sw,
}

impl FromStr for HexDirection {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<HexDirection, Self::Err> {
        match &*s.to_ascii_lowercase() {
            "n" => Ok(HexDirection::N),
            "ne" => Ok(HexDirection::Ne),
            "nw" => Ok(HexDirection::Nw),
            "s" => Ok(HexDirection::S),
            "se" => Ok(HexDirection::Se),
            "sw" => Ok(HexDirection::Sw),
            _ => Err("Could not parse as HexDirection"),
        }
    }
}


/// Track a location in a hex grid
///
/// Every cell in a hex grid can be uniquely identified by a pair
/// of coordinates in a two-axis system; there are at least four
/// different types of two-axis systems available. However, it's
/// more efficient to use a cubical system, so that's what we do here.
///
/// Cubical coordinate systems have a unique address for every hex
/// if constrained such that the sum of axis values always equals 0.
///
/// See https://www.redblobgames.com/grids/hexagons/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HexPosition {
    east: isize,
    northwest: isize,
    southwest: isize,
}

impl HexPosition {
    pub fn new() -> HexPosition {
        HexPosition {
            east: 0,
            northwest: 0,
            southwest: 0,
        }
    }

    pub fn step(&self, direction: HexDirection) -> HexPosition {
        use HexDirection::*;
        match direction {
            N => {
                HexPosition {
                    northwest: self.northwest + 1,
                    southwest: self.southwest - 1,
                    ..*self
                }
            }
            S => {
                HexPosition {
                    northwest: self.northwest - 1,
                    southwest: self.southwest + 1,
                    ..*self
                }
            }
            Ne => {
                HexPosition {
                    east: self.east + 1,
                    southwest: self.southwest - 1,
                    ..*self
                }
            }
            Sw => {
                HexPosition {
                    east: self.east - 1,
                    southwest: self.southwest + 1,
                    ..*self
                }
            }
            Nw => {
                HexPosition {
                    east: self.east - 1,
                    northwest: self.northwest + 1,
                    ..*self
                }
            }
            Se => {
                HexPosition {
                    east: self.east + 1,
                    northwest: self.northwest - 1,
                    ..*self
                }
            }
        }
    }

    /// Generate all possible axial coordintes from this position.
    ///
    /// Axial coordinates are generated from cubical coordinates
    /// by dropping one of the cubical axes.
    ///
    /// As there are three axes, there are three potential axial
    /// coordinate systems. This function drops axes in the sequence
    /// `east`, `northwest`, `southwest`. Therefore, the output represents
    /// the axial systems:
    ///
    /// 1. `(northwest, southwest)`
    /// 2. `(east, southwest)`
    /// 3. `(east, northwest)`
    pub fn to_axial(&self) -> [(isize, isize); 3] {
        [
            (self.northwest, self.southwest),
            (self.east, self.southwest),
            (self.east, self.northwest),
        ]
    }

    /// Find the minimum number of steps required to navigate to the origin.
    ///
    /// From any point in a hex grid, it's possible to navigate to the origin
    /// by repeatedly moving in at most two directions. Therefore, the minimal
    /// number of steps to the origin must always be the sum of the absolute values
    /// of the axes in one of the possible axial systems.
    pub fn min_steps_to_origin(&self) -> isize {
        self.to_axial()
            .iter()
            .map(|&(left, right)| left.abs() + right.abs())
            .min()
            // unwrap is safe because we know that to_axial always returns three values
            .unwrap()
    }
}

impl Add<HexDirection> for HexPosition {
    type Output = HexPosition;
    fn add(self, other: HexDirection) -> HexPosition {
        self.step(other)
    }
}

impl<'a> Add<&'a HexDirection> for HexPosition {
    type Output = HexPosition;
    fn add(self, other: &'a HexDirection) -> HexPosition {
        self.step(*other)
    }
}

impl Sum<HexDirection> for HexPosition {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = HexDirection>,
    {
        let mut position = HexPosition::new();
        for direction in iter {
            position = position + direction;
        }
        position
    }
}

impl<'a> Sum<&'a HexDirection> for HexPosition {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a HexDirection>,
    {
        let mut position = HexPosition::new();
        for direction in iter {
            position = position + direction;
        }
        position
    }
}
