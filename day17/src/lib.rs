pub const TWENTY_SEVENTEEN: usize = 2017;
pub const FIFTY_MILLION: usize = 50_000_000;

type List<T> = Vec<T>;

pub struct Spinner {
    index: usize,
    items: List<usize>,
    next_insert: usize,
}

impl Spinner {
    pub fn new() -> Spinner {
        let mut items = List::with_capacity(FIFTY_MILLION);
        items.insert(0, 0);
        Spinner {
            index: 0,
            items: items,
            next_insert: 1,
        }
    }

    fn spin_insert(&mut self, steps: usize, item: usize) {
        self.index = ((self.index + steps) % self.items.len()) + 1;
        self.items.insert(self.index, item);
    }

    pub fn insert_until(&mut self, until: usize, steps: usize) {
        for item in self.next_insert..(until + 1) {
            self.spin_insert(steps, item);
        }
        self.next_insert = until + 1;
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_items(&self) -> &List<usize> {
        &self.items
    }
}

pub struct FastSpinner {
    index: usize,
    items_len: usize,
    next_insert: usize,
    after_zero: usize,
}

impl FastSpinner {
    pub fn new() -> FastSpinner {
        FastSpinner {
            index: 0,
            items_len: 1,
            next_insert: 1,
            after_zero: 0,
        }
    }

    fn spin_insert(&mut self, steps: usize, item: usize) {
        self.index = ((self.index + steps) % self.items_len) + 1;
        self.items_len += 1;
        if self.index == 1 {
            self.after_zero = item;
        }
    }

    pub fn insert_until(&mut self, until: usize, steps: usize) {
        for item in self.next_insert..(until + 1) {
            self.spin_insert(steps, item);
        }
        self.next_insert = until + 1;
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_after_zero(&self) -> usize {
        self.after_zero
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_spinner() {
        const INPUT: usize = 349;
        let mut spinner = Spinner::new();
        spinner.insert_until(TWENTY_SEVENTEEN, INPUT);
        let zero_index = spinner.get_items().iter().position(|&i| i == 0).unwrap();
        let spinner_after_zero = spinner.get_items()[zero_index + 1];

        let mut fspinner = FastSpinner::new();
        fspinner.insert_until(TWENTY_SEVENTEEN, INPUT);
        assert_eq!(fspinner.get_after_zero(), spinner_after_zero);
    }
}
