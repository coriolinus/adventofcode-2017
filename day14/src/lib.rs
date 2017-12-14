#[macro_use]
extern crate util;

extern crate day10;
use day10::hash;

use std::str;

/// Get the number of bits set in the input value
///
/// Used without any attempt to grok from
/// https://stackoverflow.com/a/109025/504550
#[inline]
pub fn hamming_weight(mut i: u32) -> u32 {
    i = i - ((i >> 1) & 0x55555555);
    i = (i & 0x33333333) + ((i >> 2) & 0x33333333);
    (((i + (i >> 4)) & 0x0F0F0F0F).wrapping_mul(0x01010101)) >> 24
}

pub fn count_bits_of_hashes_for(key: &str) -> u32 {
    let mut hamming_sum = 0;
    for row in 0..128 {
        let input_str = format!("{}-{}", key, row);
        let row_hash = hash(&input_str.as_bytes());
        debug_println!("{}:", input_str);
        debug_println!("  row_hash: {}", row_hash);
        hamming_sum += row_hash
            .as_bytes()
            .chunks(8) // 8 chars represent 32 bits, so will fit into a u32
            .map(|chunk| {
                hamming_weight(
                    u32::from_str_radix(
                        // chunk is a &[u8] whose origin is a utf-8 string comprised
                        // of characters in the set `[0-9a-f]`.
                        //
                        // All of those characters are 1 byte wide, so this unsafe block
                        // is sound.
                        unsafe { str::from_utf8_unchecked(chunk) },
                        16
                        // we know that these characters are parseable as u32
                    ).unwrap(),
                )
            })
            .sum::<u32>();

    }
    hamming_sum
}

fn bitvec(key: &str) -> Vec<Vec<bool>> {
    const ROWS: usize = 128;
    let mut bits = Vec::with_capacity(ROWS);
    for row in 0..ROWS {
        let input_str = format!("{}-{}", key, row);
        let row_hash = hash(&input_str.as_bytes());
        let mut row = Vec::with_capacity(4 * 32);
        for ch in row_hash.chars() {
            let bits = match ch {
                '0' => [false, false, false, false],
                '1' => [false, false, false, true],
                '2' => [false, false, true, false],
                '3' => [false, false, true, true],
                '4' => [false, true, false, false],
                '5' => [false, true, false, true],
                '6' => [false, true, true, false],
                '7' => [false, true, true, true],
                '8' => [true, false, false, false],
                '9' => [true, false, false, true],
                'a' => [true, false, true, false],
                'b' => [true, false, true, true],
                'c' => [true, true, false, false],
                'd' => [true, true, false, true],
                'e' => [true, true, true, false],
                'f' => [true, true, true, true],
                _ => panic!("found unparseable char in row hash"),
            };
            row.extend(&bits);
        }
        bits.push(row);
    }
    bits
}

pub fn bitvec_true_count(key: &str) -> usize {
    bitvec(key)
        .iter()
        .map(|row| row.iter().filter(|&&b| b).count())
        .sum()
}

#[derive(Clone, Copy)]
struct Regional {
    value: bool,
    region: Option<usize>,
}

impl Regional {
    fn new(value: bool) -> Regional {
        Regional {
            value: value,
            region: None,
        }
    }
}

type Rfield = Vec<Vec<Regional>>;

fn to_rfield(bv: Vec<Vec<bool>>) -> Rfield {
    let mut rv = Vec::with_capacity(bv.len());
    rv.extend(bv.into_iter().map(|row| {
        let mut r_row = Vec::with_capacity(row.len());
        r_row.extend(row.into_iter().map(|v| Regional::new(v)));
        r_row
    }));
    rv
}

fn assign_group(rfield: &mut Rfield, row: usize, col: usize, group: usize) {
    if rfield[row][col].value && rfield[row][col].region.is_none() {
        rfield[row][col].region = Some(group);
        if row > 0 {
            assign_group(rfield, row - 1, col, group);
        }
        if col > 0 {
            assign_group(rfield, row, col - 1, group);
        }
        if row + 1 < rfield.len() {
            assign_group(rfield, row + 1, col, group);
        }
        if col + 1 < rfield[row].len() {
            assign_group(rfield, row, col + 1, group);
        }
    }
}

pub fn region_count(key: &str) -> usize {
    let mut rfield = to_rfield(bitvec(key));
    let mut group = 0;
    for r_idx in 0..rfield.len() {
        for c_idx in 0..rfield[r_idx].len() {
            if rfield[r_idx][c_idx].value && rfield[r_idx][c_idx].region.is_none() {
                group += 1;
                assign_group(&mut rfield, r_idx, c_idx, group);
            }
        }
    }
    group
}
