//! Manually parse the problem input
//!
//! I could have written another parser to handle it, but the
//! input is relatively short, and it's simpler and faster to
//! just manually parse it.

use super::{Instruction, StateName, States};

pub const FIRST_STATE: StateName = 'A';
pub const CHECKSUM_AFTER: usize = 12994925;

pub fn states() -> States {
    use Direction::*;
    let mut ret = States::with_capacity(6);
    ret.insert(
        'A',
        [
            Instruction::new_i(1, Right, 'B').unwrap(),
            Instruction::new_i(0, Left, 'F').unwrap(),
        ],
    );
    ret.insert(
        'B',
        [
            Instruction::new_i(0, Right, 'C').unwrap(),
            Instruction::new_i(0, Right, 'D').unwrap(),
        ],
    );
    ret.insert(
        'C',
        [
            Instruction::new_i(1, Left, 'D').unwrap(),
            Instruction::new_i(1, Right, 'E').unwrap(),
        ],
    );
    ret.insert(
        'D',
        [
            Instruction::new_i(0, Left, 'E').unwrap(),
            Instruction::new_i(0, Left, 'D').unwrap(),
        ],
    );
    ret.insert(
        'E',
        [
            Instruction::new_i(0, Right, 'A').unwrap(),
            Instruction::new_i(1, Right, 'C').unwrap(),
        ],
    );
    ret.insert(
        'F',
        [
            Instruction::new_i(1, Left, 'A').unwrap(),
            Instruction::new_i(1, Right, 'A').unwrap(),
        ],
    );
    ret
}
