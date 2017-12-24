use std::collections::HashMap;

pub mod parser;

pub type RegisterID = char;
pub type RegisterValue = i64;
type Registers = HashMap<RegisterID, RegisterValue>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueHolder {
    Register(RegisterID),
    Number(RegisterValue),
}

impl ValueHolder {
    fn value(&self, registers: &mut Registers) -> RegisterValue {
        use ValueHolder::*;
        match *self {
            Register(id) => *registers.entry(id).or_insert(0),
            Number(value) => value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Set(RegisterID, ValueHolder),
    Sub(RegisterID, ValueHolder),
    Mul(RegisterID, ValueHolder),
    Jnz(ValueHolder, ValueHolder),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineError {
    IPOutOfBounds,
    ReadBlock,
}

pub struct Machine<'instructions> {
    instructions: &'instructions [Instruction],
    instruction_pointer: RegisterValue,
    registers: Registers,
}

impl<'instructions> Machine<'instructions> {
    pub fn new(instructions: &'instructions [Instruction], debug: bool) -> Machine<'instructions> {
        Machine {
            instructions: instructions,
            instruction_pointer: 0,
            registers: {
                let mut h = HashMap::new();
                if !debug {
                    h.insert('a', 1);
                }
                h
            },
        }
    }

    /// Execute a single instruction
    fn execute(&mut self) -> Result<(Instruction, RegisterValue), MachineError> {
        if !(self.instruction_pointer >= 0 &&
             (self.instruction_pointer as usize) < self.instructions.len()) {
            return Err(MachineError::IPOutOfBounds);
        }
        let (ip, rv) = {
            use Instruction::*;
            match self.instructions[self.instruction_pointer as usize] {
                Set(id, valueh) => {
                    let value = valueh.value(&mut self.registers);
                    self.registers.insert(id, value);
                    (self.instruction_pointer + 1, (Set(id, valueh), value))
                }
                Sub(id, valueh) => {
                    let value = valueh.value(&mut self.registers);
                    *self.registers.entry(id).or_insert(0) -= value;
                    (self.instruction_pointer + 1, (Sub(id, valueh), value))
                }
                Mul(id, valueh) => {
                    let value = valueh.value(&mut self.registers);
                    *self.registers.entry(id).or_insert(0) *= value;
                    (self.instruction_pointer + 1, (Mul(id, valueh), value))
                }
                Jnz(x, y) => {
                    if x.value(&mut self.registers) != 0 {
                        let yval = y.value(&mut self.registers);
                        let dest = self.instruction_pointer + yval;
                        (dest, (Jnz(x, y), yval))
                    } else {
                        (self.instruction_pointer + 1, (Jnz(x, y), 1))
                    }
                }
            }
        };
        self.instruction_pointer = ip;
        Ok(rv)
    }

    pub fn count_mul(&mut self) -> usize {
        let mut invocations = 0;
        while let Ok((instruction, _)) = self.execute() {
            if let Instruction::Mul(_, _) = instruction {
                invocations += 1;
            }
        }
        invocations
    }

    pub fn run(&mut self) {
        while let Ok(_) = self.execute() {}
    }

    pub fn registers(&self) -> &Registers {
        &self.registers
    }
}
