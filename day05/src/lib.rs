pub struct JumpMemory {
    values: Vec<isize>,
    cursor: usize,
    steps: usize,
}

impl JumpMemory {
    pub fn new(values: &[isize]) -> JumpMemory {
        JumpMemory {
            values: Vec::from(values),
            cursor: 0,
            steps: 0,
        }
    }

    /// Perform a single jump
    ///
    /// Jumping has weird semantics here: First move the cursor, then
    /// increment the value at the cursor's previous location.
    ///
    /// Returns the state of the struct: if True, the state is valid
    /// and jumps can continue. If False, the state is invalid because
    /// the cursor has escaped the bounds of the memory.
    fn jump(&mut self) -> bool {
        let next_cursor = self.cursor as isize + self.values[self.cursor];
        self.values[self.cursor] += 1;
        let rv = next_cursor >= 0 && next_cursor < self.values.len() as isize;
        if rv {
            self.cursor = next_cursor as usize;
        }
        self.steps += 1;
        rv
    }

    /// Run until the cursor escapes the memory,
    /// and return the number of steps it took
    pub fn run(&mut self) -> usize {
        while self.jump() {}
        self.steps
    }

    /// Perform a single jump
    ///
    /// Jumping has weird semantics here: First move the cursor, then
    /// udpate the value at the cursor's previous location based on
    /// its current value.
    ///
    /// Returns the state of the struct: if True, the state is valid
    /// and jumps can continue. If False, the state is invalid because
    /// the cursor has escaped the bounds of the memory.
    fn jump2(&mut self) -> bool {
        let next_cursor = self.cursor as isize + self.values[self.cursor];
        if self.values[self.cursor] >= 3 {
            self.values[self.cursor] -= 1;
        } else {
            self.values[self.cursor] += 1;
        }
        let rv = next_cursor >= 0 && next_cursor < self.values.len() as isize;
        if rv {
            self.cursor = next_cursor as usize;
        }
        self.steps += 1;
        rv
    }

    /// Run until the cursor escapes the memory,
    /// and return the number of steps it took
    pub fn run2(&mut self) -> usize {
        while self.jump2() {}
        self.steps
    }
}
