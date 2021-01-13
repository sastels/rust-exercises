use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

fn factor_sum(n: u64) -> u64 {
    let mut n_factors: Vec<u64> = vec![];

    for f in 1..=(n / 2) {
        if n % f == 0 {
            n_factors.push(f);
        }
    }
    n_factors.iter().sum()
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        None
    } else {
        match factor_sum(num).cmp(&num) {
            Ordering::Less => Some(Classification::Deficient),
            Ordering::Equal => Some(Classification::Perfect),
            Ordering::Greater => Some(Classification::Abundant),
        }
    }
}
