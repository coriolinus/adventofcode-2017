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
    Snd(ValueHolder),
    Set(RegisterID, ValueHolder),
    Add(RegisterID, ValueHolder),
    Mul(RegisterID, ValueHolder),
    Mod(RegisterID, ValueHolder),
    Rcv(RegisterID),
    Jgz(ValueHolder, ValueHolder),
}

pub struct Machine<'a> {
    instructions: &'a [Instruction],
    instruction_pointer: RegisterValue,
    registers: Registers,
    sound_playing: Option<RegisterValue>,
}

impl<'a> Machine<'a> {
    pub fn new(instructions: &'a [Instruction]) -> Machine<'a> {
        Machine {
            instructions: instructions,
            instruction_pointer: 0,
            registers: HashMap::new(),
            sound_playing: None,
        }
    }

    /// Execute a single instruction
    fn execute(&mut self) -> Result<(Instruction, RegisterValue), ()> {
        if !(self.instruction_pointer >= 0 &&
                 (self.instruction_pointer as usize) < self.instructions.len())
        {
            return Err(());
        }
        let (ip, rv) = {
            use Instruction::*;
            match self.instructions[self.instruction_pointer as usize] {
                Snd(soundh) => {
                    let sound = soundh.value(&mut self.registers);
                    self.sound_playing = Some(sound);
                    (self.instruction_pointer + 1, (Snd(soundh), sound))
                }
                Set(id, valueh) => {
                    let value = valueh.value(&mut self.registers);
                    self.registers.insert(id, value);
                    (self.instruction_pointer + 1, (Set(id, valueh), value))
                }
                Add(id, valueh) => {
                    let value = valueh.value(&mut self.registers);
                    *self.registers.entry(id).or_insert(0) += value;
                    (self.instruction_pointer + 1, (Add(id, valueh), value))
                }
                Mul(id, valueh) => {
                    let value = valueh.value(&mut self.registers);
                    *self.registers.entry(id).or_insert(0) *= value;
                    (self.instruction_pointer + 1, (Mul(id, valueh), value))
                }
                Mod(id, valueh) => {
                    let value = valueh.value(&mut self.registers);
                    *self.registers.entry(id).or_insert(0) %= value;
                    (self.instruction_pointer + 1, (Mod(id, valueh), value))
                }
                Rcv(id) => {
                    let sound = self.sound_playing.unwrap_or(0);
                    if *self.registers.entry(id).or_insert(0) != 0 {
                        (self.instruction_pointer + 1, (Rcv(id), sound))
                    } else {
                        (self.instruction_pointer + 1, (Rcv(id), 0))
                    }
                }
                Jgz(x, y) => {
                    if x.value(&mut self.registers) > 0 {
                        let yval = y.value(&mut self.registers);
                        let dest = self.instruction_pointer + yval;
                        (dest, (Jgz(x, y), yval))
                    } else {
                        (self.instruction_pointer + 1, (Jgz(x, y), 1))
                    }
                }
            }
        };
        self.instruction_pointer = ip;
        Ok(rv)
    }

    pub fn run(&mut self) -> Option<RegisterValue> {
        while let Ok((instruction, value)) = self.execute() {
            if let Instruction::Rcv(_) = instruction {
                return Some(value);
            }
        }
        None
    }
}
