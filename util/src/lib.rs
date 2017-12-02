use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

pub fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect(
        "Something went wrong reading the file",
    );
    contents
}

pub fn tokenize<'a>(input: &'a str) -> Vec<Vec<&'a str>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter(|ref token| !token.is_empty())
                .collect::<Vec<_>>()
        })
        .filter(|ref line| !line.is_empty())
        .collect()
}

pub fn parse_as<X, Y, Input, Output>(
    input: &Y,
) -> Result<Vec<Vec<Output>>, <Output as FromStr>::Err>
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

pub fn file_as<Output>(filename: &str) -> Result<Vec<Vec<Output>>, <Output as FromStr>::Err>
where
    Output: FromStr,
{
    parse_as(&tokenize(&read_file(filename)))
}
