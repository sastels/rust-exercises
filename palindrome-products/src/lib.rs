#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    factors: Vec<(u64, u64)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome {
            factors: vec![(a, b)],
        }
    }

    pub fn value(&self) -> u64 {
        self.factors[0].0 * self.factors[0].1
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.factors.push((a, b));
    }
}

fn is_palindrome(a: u64, b: u64) -> bool {
    let product = format!("{}", a * b);
    product == product.chars().rev().collect::<String>()
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    use std::cmp::Ordering;
    let mut min_palindrome = Palindrome::new(max + 1, max + 1);
    let mut max_palindrome = Palindrome::new(0, 0);

    for a in min..=max {
        for b in a..=max {
            if is_palindrome(a, b) {
                match (a * b).cmp(&min_palindrome.value()) {
                    Ordering::Less => min_palindrome = Palindrome::new(a, b),
                    Ordering::Equal => min_palindrome.insert(a, b),
                    Ordering::Greater => (),
                }
                match (a * b).cmp(&max_palindrome.value()) {
                    Ordering::Less => (),
                    Ordering::Equal => max_palindrome.insert(a, b),
                    Ordering::Greater => max_palindrome = Palindrome::new(a, b),
                }
            }
        }
    }

    if max_palindrome.value() == 0 {
        None
    } else {
        Some((min_palindrome, max_palindrome))
    }
}
