use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = HashSet::new();
    for n in factors {
        let mut multiple: u32 = *n;
        while multiple < limit && multiple != 0 {
            multiples.insert(multiple);
            multiple += n;
        }
    }
    multiples.iter().fold(0, |sum, value| sum + value)
}
