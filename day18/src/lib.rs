use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use std::sync::RwLock;

pub mod parser;

pub type RegisterID = char;
pub type RegisterValue = i64;
type Registers = HashMap<RegisterID, RegisterValue>;
type Queue = Rc<RwLock<VecDeque<RegisterValue>>>;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineError {
    IPOutOfBounds,
    ReadBlock,
}

pub struct Machine<'instructions> {
    instructions: &'instructions [Instruction],
    instruction_pointer: RegisterValue,
    registers: Registers,
    send_q: Queue,
    recv_q: Queue,
}

impl<'instructions> Machine<'instructions> {
    pub fn new(
        instructions: &'instructions [Instruction],
        id: RegisterValue,
        send_q: Queue,
        recv_q: Queue,
    ) -> Machine<'instructions> {
        Machine {
            instructions: instructions,
            instruction_pointer: 0,
            registers: {
                let mut h = HashMap::new();
                h.insert('p', id);
                h
            },
            send_q: send_q,
            recv_q: recv_q,
        }
    }

    /// Execute a single instruction
    fn execute(&mut self) -> Result<(Instruction, RegisterValue), MachineError> {
        if !(self.instruction_pointer >= 0 &&
                 (self.instruction_pointer as usize) < self.instructions.len())
        {
            return Err(MachineError::IPOutOfBounds);
        }
        let (ip, rv) = {
            use Instruction::*;
            match self.instructions[self.instruction_pointer as usize] {
                Snd(messageh) => {
                    let message = messageh.value(&mut self.registers);
                    self.send_q
                        .write()
                        .expect("couldn't get send lock")
                        .push_back(message);
                    (self.instruction_pointer + 1, (Snd(messageh), message))
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
                    if let Some(message) = self.recv_q
                        .write()
                        .expect("couldn't get receive lock")
                        .pop_front()
                    {
                        self.registers.insert(id, message);
                        (self.instruction_pointer + 1, (Rcv(id), message))
                    } else {
                        return Err(MachineError::ReadBlock);
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

pub struct Duet<'instructions> {
    machine_0: Machine<'instructions>,
    machine_1: Machine<'instructions>,
    send_count_1: usize,
}

impl<'instructions> Duet<'instructions> {
    pub fn new(instructions: &'instructions [Instruction]) -> Duet {
        let queue_0 = Rc::new(RwLock::new(VecDeque::new()));
        let queue_1 = Rc::new(RwLock::new(VecDeque::new()));
        let machine_0 = Machine::new(instructions, 0, queue_1.clone(), queue_0.clone());
        let machine_1 = Machine::new(instructions, 1, queue_0.clone(), queue_1.clone());
        Duet {
            machine_0: machine_0,
            machine_1: machine_1,
            send_count_1: 0,
        }
    }

    pub fn run(&mut self) -> usize {
        loop {
            let result_0 = self.machine_0.execute();
            let result_1 = self.machine_1.execute();
            if let Ok((Instruction::Snd(_), _)) = result_1 {
                self.send_count_1 += 1;
            }
            if result_0.is_err() && result_1.is_err() {
                break;
            }
        }
        self.send_count_1
    }
}
