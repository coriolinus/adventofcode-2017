pub fn captcha(digits: &[usize]) -> usize {
    let mut sum = 0;
    for index in 0..digits.len() {
        let index_next = (index + (digits.len() / 2)) % digits.len();
        if digits[index] == digits[index_next] {
            sum += digits[index];
        }
    }
    sum
}

pub fn captcha_str(input: &str) -> Option<usize> {
    input
        .chars()
        .map(|c| c.to_digit(10).map(|d| d as usize))
        .collect::<Option<Vec<usize>>>()
        .map(|digits| captcha(&digits))
}
