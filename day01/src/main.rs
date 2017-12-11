extern crate day01;
use day01::captcha_str;

extern crate util;
use util::read_file;

fn main() {
    let input = read_file("../input.txt");
    println!("Captcha for input: {:?}", captcha_str(input.trim()));
}
