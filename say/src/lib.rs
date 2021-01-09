#[macro_use]
extern crate lazy_static;
use std::cmp::Ordering::*;
use std::collections::HashMap;

lazy_static! {
    static ref DIGITS: HashMap<u64, String> = {
        let mut m = HashMap::new();
        m.insert(0, "zero".to_string());
        m.insert(1, "one".to_string());
        m.insert(2, "two".to_string());
        m.insert(3, "three".to_string());
        m.insert(4, "four".to_string());
        m.insert(5, "five".to_string());
        m.insert(6, "six".to_string());
        m.insert(7, "seven".to_string());
        m.insert(8, "eight".to_string());
        m.insert(9, "nine".to_string());
        m
    };
}

lazy_static! {
    static ref TENS: HashMap<u64, String> = {
        let mut m = HashMap::new();
        m.insert(10, "ten".to_string());
        m.insert(20, "twenty".to_string());
        m.insert(30, "thirty".to_string());
        m.insert(40, "forty".to_string());
        m.insert(50, "fifty".to_string());
        m.insert(60, "sixty".to_string());
        m.insert(70, "seventy".to_string());
        m.insert(80, "eighty".to_string());
        m.insert(90, "ninety".to_string());
        m
    };
}

lazy_static! {
    static ref TEENS: HashMap<u64, String> = {
        let mut m = HashMap::new();
        m.insert(10, "ten".to_string());
        m.insert(11, "eleven".to_string());
        m.insert(12, "twelve".to_string());
        m.insert(13, "thirteen".to_string());
        m.insert(14, "fourteen".to_string());
        m.insert(15, "fifteen".to_string());
        m.insert(16, "sixteen".to_string());
        m.insert(17, "seventeen".to_string());
        m.insert(18, "eighteen".to_string());
        m.insert(19, "nineteen".to_string());
        m
    };
}

fn digit(n: u64) -> String {
    DIGITS.get(&n).unwrap().clone()
}

fn teen(n: u64) -> String {
    TEENS.get(&n).unwrap().clone()
}

fn ten(n: u64) -> String {
    TENS.get(&n).unwrap().clone()
}

fn encode_below_20(n: u64) -> String {
    match n.cmp(&10) {
        Greater => teen(n),
        Equal => "ten".to_string(),
        Less => digit(n),
    }
}

fn encode_below_100(n: u64) -> String {
    if n < 20 {
        return encode_below_20(n);
    }
    let tens = (n / 10) * 10;
    let ones = n % 10;
    if ones == 0 {
        ten(tens)
    } else {
        format!("{}-{}", ten(tens), digit(ones))
    }
}

fn encode_below_1000(n: u64) -> String {
    let mut n = n;
    let mut encoded = "".to_string();

    // strip hundreds
    let hundreds = n / 100;
    n -= hundreds * 100;
    if hundreds > 0 {
        encoded = format!("{} hundred", digit(hundreds));
    }

    // below 100
    if n != 0 {
        encoded = format!("{} {}", encoded, encode_below_100(n));
    }
    encoded.trim().to_string()
}

pub fn encode(n: u64) -> String {
    let mut n = n;
    let mut encoded = "".to_string();

    for (text, val) in &[
        ("quintillion", 1_000_000_000_000_000_000),
        ("quadrillion", 1_000_000_000_000_000),
        ("trillion", 1_000_000_000_000),
        ("billion", 1_000_000_000),
        ("million", 1_000_000),
        ("thousand", 1_000),
        ("", 1),
    ] {
        let x = n / val;
        n -= x * val;
        if x > 0 {
            encoded = format!("{} {} {}", encoded, encode_below_1000(x), text)
                .trim()
                .to_string();
        }
    }

    if encoded.is_empty() {
        encoded = "zero".to_string();
    }
    encoded
}
