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
        m.insert(40, "fourty".to_string());
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
    println!("digit {}", n);
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
        Greater => return teen(n),
        Equal => return "ten".to_string(),
        Less => return digit(n),
    };
}

fn encode_below_100(n: u64) -> String {
    // less than 20
    if n < 20 {
        return encode_below_20(n);
    }

    let tens = (n / 10) * 10;
    let ones = n % 10;

    if ones == 0 {
        return ten(tens);
    } else {
        return format!("{}-{}", ten(tens), digit(ones));
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
    return encoded.trim().to_string();
}

fn partial_encode(text: &str, val: u64, n: &mut u64) -> String {
    let x = *n / val;
    *n -= x * val;
    if x > 0 {
        return format!("{} {}", encode_below_1000(x), text)
            .trim()
            .to_string();
    } else {
        return "".to_string();
    }
}

pub fn encode(n: u64) -> String {
    let mut n = n;
    let mut encoded = "".to_string();

    // billions
    encoded = format!(
        "{} {}",
        encoded,
        partial_encode("billion", 1_000_000_000, &mut n)
    )
    .trim()
    .to_string();

    let billions = n / 1_000_000_000;
    n -= billions * 1_000_000_000;
    if billions > 0 {
        encoded = format!("{} {} billion", encoded, encode_below_1000(billions))
            .trim()
            .to_string();
    }

    // millions
    let millions = n / 1_000_000;
    n -= millions * 1_000_000;
    if millions > 0 {
        encoded = format!("{} {} million", encoded, encode_below_1000(millions))
            .trim()
            .to_string();
    }

    // thousands
    let thousands = n / 1000;
    n -= thousands * 1000;
    if thousands > 0 {
        encoded = format!("{} {} thousand", encoded, encode_below_1000(thousands))
            .trim()
            .to_string();
    }

    // below 1000
    if n > 0 {
        encoded = format!("{} {}", encoded, encode_below_1000(n))
            .trim()
            .to_string();
    }

    if encoded.is_empty() {
        encoded = "zero".to_string();
    }
    encoded
}
