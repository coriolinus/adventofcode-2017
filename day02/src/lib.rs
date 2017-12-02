//! Corruption Checksum
//!
//! The spreadsheet consists of rows of apparently-random numbers. To make sure the
//! recovery process is on the right track, they need you to calculate the
//! spreadsheet's checksum. For each row, determine the difference between the largest
//! value and the smallest value; the checksum is the sum of all of these differences.
//!
//! For example, given the following spreadsheet:
//!
//! ```text
//! 5 1 9 5
//! 7 5 3
//! 2 4 6 8
//! ```
//!
//! - The first row's largest and smallest values are 9 and 1, and their difference is 8.
//! - The second row's largest and smallest values are 7 and 3, and their difference is 4.
//! - The third row's difference is 6.
//!
//! In this example, the spreadsheet's checksum would be `8 + 4 + 6 = 18`.

use std::ops::{Div, Rem, Sub};
use std::iter::Sum;

extern crate num_traits;
use num_traits::Zero;

pub fn checksum<X, Y, T>(sheet: &Y) -> T
where
    Y: AsRef<[X]>,
    X: AsRef<[T]>,
    T: Sub<Output = T> + Sum<T> + Ord + Zero + Copy,
{
    sheet
        .as_ref()
        .iter()
        .map(|row| match (
            row.as_ref().iter().max(),
            row.as_ref().iter().min(),
        ) {
            (Some(max), Some(min)) => *max - *min,
            _ => T::zero(),
        })
        .sum()
}

fn even_division<X, T>(row: &X) -> T
where
    X: AsRef<[T]>,
    T: Div<Output = T> + Rem<Output = T> + Zero + PartialEq + Copy,
{
    let row = row.as_ref();
    for divisor_index in 0..row.len() {
        for dividend_index in 0..row.len() {
            if dividend_index != divisor_index {
                if row[divisor_index] % row[dividend_index] == T::zero() {
                    return row[divisor_index] / row[dividend_index];
                }
            }
        }
    }
    panic!("No even divisors found")
}

pub fn divisible_checksum<X, Y, T>(sheet: &Y) -> T
where
    Y: AsRef<[X]>,
    X: AsRef<[T]>,
    T: Sum<T> + Div<Output = T> + Rem<Output = T> + Zero + PartialEq + Copy,
{
    sheet.as_ref().iter().map(even_division).sum()
}
