/// Determine whether a sentence is a pangram.
use itertools::Itertools;

pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .unique()
        .count()
        == 26
}
