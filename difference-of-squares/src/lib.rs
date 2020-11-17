pub fn square_of_sum(n: u32) -> u32 {
    let sum = (1..n + 1).fold(0, |sum, value| sum + &value);
    sum * sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..n + 1).map(|n| n * n).fold(0, |sum, value| sum + &value)
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
