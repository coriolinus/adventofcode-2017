extern crate day01;
use day01::captcha_str;

fn main() {
    let input_str = include_str!("../input.txt").trim();
    println!("Captcha for input: {:?}", captcha_str(&input_str));
}
