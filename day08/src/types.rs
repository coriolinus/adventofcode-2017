pub type Register = i64;

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Inc,
    Dec,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Gt,
    Lt,
    Gte,
    Lte,
    Eq,
    Ne,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction<'a> {
    register: &'a str,
    operation: Operation,
    qty: Register,
    compare_register: &'a str,
    comparison: Comparison,
    compare_qty: Register,
}

impl<'a> Instruction<'a> {
    pub fn new(
        register: &'a str,
        operation: Operation,
        qty: Register,
        compare_register: &'a str,
        comparison: Comparison,
        compare_qty: Register,
    ) -> Instruction<'a> {
        Instruction {
            register: register,
            operation: operation,
            qty: qty,
            compare_register: compare_register,
            comparison: comparison,
            compare_qty: compare_qty,
        }
    }
}
