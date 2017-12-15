use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use std::mem;

pub mod modular_math;

pub type Tokenized<T> = Vec<Vec<T>>;
pub type ParsedTokens<T> = Result<Tokenized<T>, <T as FromStr>::Err>;

/// Read a file to a string
///
/// Panic if anything goes wrong
pub fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect(
        "Something went wrong reading the file",
    );
    contents
}

/// Convert a string to a `Vec<Vec<&str>>`
///
/// The outer vector contains the lines of the string, split by newlines
/// The inner vector contains the tokens of the string, split by whitespace
///
/// Empty lines and empty whitespace are discarded
pub fn tokenize<'a>(input: &'a str) -> Tokenized<&'a str> {
    tokenize_by(input, |line| {
        line.split_whitespace()
            .filter(|ref token| !token.is_empty())
            .collect::<Vec<_>>()
    })
}

/// Use a given tokenization function to convert a string to a `Vec<Vec<&str>>`
///
/// The outer vector contains the lines of the string, split by newlines
/// The inner vector contains the tokens of the string, split by the supplied tokenization function
///
/// Empty lines are discarded
pub fn tokenize_by<'a, LineTokenizer>(
    input: &'a str,
    line_tokenizer: LineTokenizer,
) -> Tokenized<&'a str>
where
    LineTokenizer: Fn(&'a str) -> Vec<&'a str>,
{
    input
        .lines()
        .map(line_tokenizer)
        .filter(|ref line| !line.is_empty())
        .collect()
}

/// Parse a `Vec<Vec<&str>>` -> `Result<Vec<Vec<Output>>, ParseError>`
pub fn parse_as<X, Y, Input, Output>(input: &Y) -> ParsedTokens<Output>
where
    Y: AsRef<[X]>,
    X: AsRef<[Input]>,
    Input: AsRef<str>,
    Output: FromStr,
{
    input
        .as_ref()
        .iter()
        .map(|row| {
            row.as_ref()
                .iter()
                .map(|token| token.as_ref().parse::<Output>())
                .collect::<Result<Vec<Output>, _>>()
        })
        .collect()
}

/// Read the file whose path is given, tokenize it, and parse the tokens as the specified type
pub fn file_as<Output>(filename: &str) -> ParsedTokens<Output>
where
    Output: FromStr,
{
    parse_as(&tokenize(&read_file(filename)))
}

/// Read the file whose path is given, tokenize it by the given line tokenization function,
/// and parse the tokens as the specified type
pub fn file_as_by<Output, LineTokenizer>(
    filename: &str,
    line_tokenizer: LineTokenizer,
) -> ParsedTokens<Output>
where
    LineTokenizer: Fn(&str) -> Vec<&str>,
    Output: FromStr,
{
    parse_as(&tokenize_by(&read_file(filename), line_tokenizer))
}

/// Convert a `Vec<Vec<T>>` -> `Vec<T>`
///
/// For each row of the outer vector, if the inner vector is not empty, takes the first item.
///
/// Useful if the input file is many lines with a single value per line
pub fn flatten<T: Default>(input: Tokenized<T>) -> Vec<T> {
    input
        .into_iter()
        .filter(|sl| sl.len() > 0)
        .map(|mut sl| {
            let mut val = T::default();
            mem::swap(&mut sl[0], &mut val);
            val
        })
        .collect()
}

/// like `println!`, but only emits output in a debug run
#[macro_export]
macro_rules! debug_println {
    ($($item:expr,)+) => {
        debug_println!($($item),+);
    };
    ($($item:expr),+) => {
        #[cfg(debug_assertions)]
        println!($($item),+);
    };
}
