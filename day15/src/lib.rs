extern crate util;
use util::modular_math::mul_mod;

pub struct Generator {
    prev_value: u64,
    factor: u64,
    quotrem: u64,
}

impl Iterator for Generator {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        self.prev_value = mul_mod(self.prev_value, self.factor, self.quotrem);
        Some(self.prev_value)
    }
}

impl Generator {
    pub fn new(prev: u64, factor: u64, quotrem: u64) -> Generator {
        Generator {
            prev_value: prev,
            factor: factor,
            quotrem: quotrem,
        }
    }

    pub fn generator_a(initial: u64) -> Generator {
        Generator::new(initial, 16807, 2147483647)
    }

    pub fn generator_b(initial: u64) -> Generator {
        Generator::new(initial, 48271, 2147483647)
    }
}

const LOW_16: u64 = 0xffff;

pub fn judge<Ia, Ib>(a: Ia, b: Ib, rounds: usize) -> usize
where
    Ia: Iterator<Item = u64>,
    Ib: Iterator<Item = u64>,
{
    a.zip(b)
        .take(rounds)
        .filter(|&(a, b)| a & LOW_16 == b & LOW_16)
        .count()
}
