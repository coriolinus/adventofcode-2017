use std::collections::{HashMap, VecDeque};

pub mod input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bit(bool);

impl From<Bit> for usize {
    fn from(rhs: Bit) -> usize {
        match rhs {
            Bit(false) => 0,
            Bit(true) => 1,
        }
    }
}

impl Default for Bit {
    fn default() -> Bit {
        Bit(false)
    }
}

pub type StateName = char;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Right,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    write: Bit,
    move_in: Direction,
    continue_with: StateName,
}

impl Instruction {
    pub fn new(write: Bit, move_in: Direction, continue_with: StateName) -> Instruction {
        Instruction {
            write: write,
            move_in: move_in,
            continue_with: continue_with,
        }
    }

    pub fn new_i(write: u8, move_in: Direction, continue_with: StateName) -> Option<Instruction> {
        let bit = match write {
            0 => Some(Bit(false)),
            1 => Some(Bit(true)),
            _ => None,
        }?;
        Some(Instruction::new(bit, move_in, continue_with))
    }
}

pub type States = HashMap<StateName, [Instruction; 2]>;

pub struct TuringMachine {
    tape: VecDeque<Bit>,
    cursor: usize,
    state: StateName,
    states: States,
}

impl TuringMachine {
    pub fn new(initial: StateName, states: States) -> TuringMachine {
        let mut tape = VecDeque::new();
        tape.push_back(Bit::default());

        TuringMachine {
            tape: tape,
            cursor: 0,
            state: initial,
            states: states,
        }
    }

    fn left(&mut self) {
        if self.cursor == 0 {
            self.tape.push_front(Bit::default());
        } else {
            self.cursor -= 1;
        }
    }

    fn right(&mut self) {
        if self.cursor + 1 == self.tape.len() {
            self.tape.push_back(Bit::default());
        }
        self.cursor += 1;
    }

    pub fn execute(&mut self) {
        let instruction: Instruction = self.states.get(&self.state).expect("unexpected state seek")
            [usize::from(self.tape[self.cursor])];
        self.tape[self.cursor] = instruction.write;
        match instruction.move_in {
            Direction::Left => self.left(),
            Direction::Right => self.right(),
        }
        self.state = instruction.continue_with;
    }

    pub fn checksum(&self) -> usize {
        self.tape.iter().map(|b| usize::from(*b)).sum()
    }
}
