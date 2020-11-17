use std::convert::TryInto;

pub fn is_armstrong_number(num: u32) -> bool {
    let digits_s: Vec<char> = num.to_string().chars().collect();
    let digits: Vec<u32> = digits_s
        .iter()
        .map(|s| s.to_string().parse().unwrap())
        .collect();

    let num_digits: u32 = digits.len().try_into().unwrap();
    let sum = digits
        .iter()
        .map(|d| d.pow(num_digits))
        .fold(0, |sum, value| sum + &value);

    sum == num
}
