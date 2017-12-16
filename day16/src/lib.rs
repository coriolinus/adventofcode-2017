use std::str::FromStr;
use std::collections::VecDeque;

pub const MIN_CHAR: char = 'a';
pub const MAX_CHAR: char = 'p';

pub enum Instruction {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl FromStr for Instruction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Instruction, &'static str> {
        use Instruction::*;
        match s.as_bytes()[0] as char {
            's' => Ok(Spin(s[1..].parse().map_err(|_| "Couldn't parse spin")?)),
            'x' => {
                let mut parts = s[1..].split('/');
                let a = parts
                    .next()
                    .ok_or("no initial value in exchange")?
                    .parse()
                    .map_err(|_| "couldn't parse instruction part one as usize")?;
                let b = parts
                    .next()
                    .ok_or("no second value in exchange")?
                    .parse()
                    .map_err(|_| "couldn't parse instruction part two as usize")?;
                if parts.next().is_some() {
                    return Err("Too many parts in exchange");
                }
                Ok(Exchange(a, b))
            }
            'p' => {
                let bstr = s[1..].as_bytes();
                if bstr.len() == 3 && bstr[1] == '/' as u8 && bstr[0] >= MIN_CHAR as u8 &&
                    bstr[0] <= MAX_CHAR as u8 && bstr[2] >= MIN_CHAR as u8 &&
                    bstr[2] <= MAX_CHAR as u8
                {
                    Ok(Partner(bstr[0] as char, bstr[2] as char))
                } else {
                    Err("failed to parse partner instruction")
                }
            }
            _ => Err("invalid instruction"),
        }
    }
}

fn programs() -> VecDeque<char> {
    let num_programs = MAX_CHAR as u8 - MIN_CHAR as u8;
    let mut programs = VecDeque::with_capacity(num_programs as usize);
    for idx in 0..(num_programs + 1) {
        programs.push_back((MIN_CHAR as u8 + idx) as char);
    }
    programs
}

fn generate_output(positions: &VecDeque<char>) -> String {
    let mut output = String::with_capacity(positions.len());
    for ch in positions {
        output.push(*ch);
    }
    output
}

pub fn dance(moves: &[Instruction]) -> String {
    let mut positions = programs();
    dance_with(moves, &mut positions);
    generate_output(&positions)
}

pub fn dance_repeat(moves: &[Instruction], count: usize) -> String {
    let mut positions = programs();
    match count {
        n if n < 1000 => {
            for _ in 0..n {
                dance_with(moves, &mut positions);
            }
        }
        _ => {
            // If there is a cycle but its length is more than half the
            // total iteration count, we don't save any time, so we may as well
            // just find up to that many
            match find_cycle(moves, &mut positions, count / 2) {
                Some(cycle) => {
                    // positions is in initial state already
                    for _ in 0..(count % cycle) {
                        dance_with(moves, &mut positions);
                    }
                }
                None => {
                    // not just count/2 to handle odd counts
                    for _ in 0..(count - (count / 2)) {
                        dance_with(moves, &mut positions);
                    }
                }
            }
        }
    }
    generate_output(&positions)
}

pub fn dance_with(moves: &[Instruction], programs: &mut VecDeque<char>) {
    use Instruction::*;
    for dance_move in moves {
        match *dance_move {
            Spin(steps) => {
                for _ in 0..steps {
                    // we know that we can pop from the back because no operation here
                    // actually reduces the length of the programs
                    let moved_ch = programs.pop_back().unwrap();
                    programs.push_front(moved_ch);
                }
            }
            Exchange(a, b) => programs.swap(a, b),
            Partner(a, b) => {
                // unwrap is safe here because we check at parse time that both
                // chars in the position function are in the range of allowed chars
                let a_idx = programs.iter().position(|&c| c == a).unwrap();
                let b_idx = programs.iter().position(|&c| c == b).unwrap();
                programs.swap(a_idx, b_idx);
            }
        }
    }
}

/// Search for a cycle by repeatedly applying `dance_with`, up to `limit` times.
///
/// If found, returns `Some(cycle_len)`; otherwise `None`.
fn find_cycle(
    moves: &[Instruction],
    positions: &mut VecDeque<char>,
    limit: usize,
) -> Option<usize> {
    let initial_positions = positions.clone();
    for idx in 1..(limit + 1) {
        dance_with(moves, positions);
        if positions == &initial_positions {
            return Some(idx);
        }
    }
    None
}

#[allow(unused)]
fn generate_translation<I: Iterator<Item = char>>(positions: I) -> Vec<usize> {
    let mut table: Vec<_> = positions
        .enumerate()
        .map(|(idx, ch)| (ch as u8 - MIN_CHAR as u8, idx))
        .collect();
    table.sort();
    table.iter().map(|&(_, idx)| idx).collect()
}

/// apply the translation to a queue of positions
///
/// must supply a buffer of the appropriate type; this prevents unnecessary allocations
#[allow(unused)]
fn apply_translation<T>(
    positions: &mut VecDeque<T>,
    mut buffer: &mut VecDeque<T>,
    translation: &[usize],
) where
    T: Copy + Default,
{
    // fill the buffer with 'blank' values
    buffer.clear();
    for _ in 0..positions.len() {
        buffer.push_back(T::default());
    }

    for (p_idx, &t_idx) in translation.iter().enumerate() {
        buffer[t_idx] = positions[p_idx];
    }

    ::std::mem::swap(positions, &mut buffer);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    /// prove that translation table approaches don't work
    ///
    /// The first assert_eq shows that we can compute a translation table from
    /// a dance move, and that we can apply it to the neutral state to reproduce
    /// the same end effect.
    ///
    /// The second assert_eq shows that dancing again, and translating again,
    /// produce different results.
    fn test_translation_table_fails() {
        println!("");
        let instructions = {
            use Instruction::*;
            [Spin(1), Exchange(3, 4), Partner('e', 'b')]
        };
        let mut positions = programs();
        dance_with(&instructions, &mut positions);
        println!("after initial dance:   {:?}", positions);
        let translation = generate_translation(positions.iter().cloned());
        println!("generated translation: {:?}", translation);

        // test that applying the translation gets the same result as the original dance
        let mut positions2 = programs();
        let mut buffer = VecDeque::with_capacity(positions.len());

        apply_translation(&mut positions2, &mut buffer, &translation);
        assert_eq!(positions, positions2);

        // test that applying the translation again gets the same result as dancing again
        dance_with(&instructions, &mut positions);
        apply_translation(&mut positions2, &mut buffer, &translation);
        assert_eq!(positions, positions2);
    }
}
