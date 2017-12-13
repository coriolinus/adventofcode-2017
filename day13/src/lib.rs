use std::collections::HashMap;
use std::cmp::max;

#[macro_use]
extern crate util;

pub struct Firewall {
    layer_depths: HashMap<usize, usize>,
    max_layer: usize,
}

impl Firewall {
    pub fn new() -> Firewall {
        Firewall {
            layer_depths: HashMap::new(),
            max_layer: 0,
        }
    }

    pub fn add_layer(&mut self, depth: usize, range: usize) {
        self.layer_depths.insert(depth, range);
        self.max_layer = max(self.max_layer, depth);
    }

    pub fn traversal_severity(&self, initial_delay: usize) -> usize {
        self.__traversal_severity(initial_delay, false)
    }

    fn __traversal_severity(&self, initial_delay: usize, short_circuit: bool) -> usize {
        let mut severity = 0;
        for (depth, time) in (initial_delay..(initial_delay + self.max_layer + 1)).enumerate() {
            debug_println!("  Depth {}, time {}:", depth, time);
            if let Some(range) = self.layer_depths.get(&depth) {
                debug_println!("    Scanner found with range: {}", *range);
                debug_println!("    Current position: {}", scanner_position(*range, time));

                if scanner_position(*range, time) == 0 {
                    severity += depth * range;
                    debug_println!(
                        "      Caught! Severity += {} (== {}):",
                        depth * range,
                        severity,
                    );
                    if short_circuit {
                        return severity + 1;
                    }
                }
            }
        }
        severity
    }


    /// Find the first delay value for which you don't get caught.
    ///
    /// Note that for unsafe inputs, this will run until it overflows
    /// usize, at which point it will panic.
    pub fn find_first_uncaught_delay(&self) -> usize {
        for delay in 0.. {
            debug_println!("Testing a delay of {}:", delay);
            if self.__traversal_severity(delay, true) == 0 {
                return delay;
            }
        }
        unreachable!("We have to assume that we'll find a working delay eventually")
    }
}

/// Compute the period of a scanner for a layer of given depth
///
/// The period is the number of time intervals between the scanner's
/// appearance at position 0. For 0 and 1 size, the period is 0 because
/// the scanner is always at position 0. For 2 size, the period is 2 because
/// there are only two positions: the surface, and one layer deep.
/// For 3 size, the period is 4: 3 as it counts down, and one more for the
/// mid position on the way back.
#[inline]
fn scanner_period(range: usize) -> usize {
    match range {
        0 => 0,
        n => (n * 2) - 2,
    }
}

/// Compute the position of the scanner given a time stamp
#[inline]
fn scanner_position(range: usize, time: usize) -> usize {
    let period_position = time % scanner_period(range);
    if period_position < range {
        period_position
    } else {
        scanner_period(range) - period_position
    }
}
