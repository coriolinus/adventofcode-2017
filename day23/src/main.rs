extern crate day23;
use day23::Machine;
use day23::parser::parse_instruction;

extern crate util;
use util::read_file;

fn main() {
    let raw_input = read_file("input.txt");
    let instructions = raw_input.lines()
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .map(|line| parse_instruction(line).expect("problem parsing line"))
        .collect::<Vec<_>>();
    let mut machine = Machine::new(&instructions, true);
    println!("mul instructions: {}", machine.count_mul());

    // figured out what the assembly of part 2 actually means:
    println!("value at h: {}", part2());
}

/// my interpretation of input program
fn part2() -> usize {
    let mut b = 109900;
    let c = 126900;
    let mut h = 0;
    let mut f;

    const STEP: usize = 17;

    while b <= c {
        f = true;
        for d in 2..b {
            if b % d == 0 {
                f = false;
                break;
            }
        }
        if !f {
            h += 1;
        }

        b += STEP;
    }

    h
}
