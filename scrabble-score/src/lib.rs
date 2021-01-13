use std::collections::HashMap;

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let mut scores: HashMap<char, u64> = HashMap::new();
    let score_data = vec![
        (vec!['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T'], 1),
        (vec!['D', 'G'], 2),
        (vec!['B', 'C', 'M', 'P'], 3),
        (vec!['F', 'H', 'V', 'W', 'Y'], 4),
        (vec!['K'], 5),
        (vec!['J', 'X'], 8),
        (vec!['Q', 'Z'], 10),
    ];
    for (letters, letter_score) in score_data {
        for c in letters {
            scores.insert(c, letter_score);
        }
    }

    word.to_ascii_uppercase()
        .chars()
        .map(|c| scores.get(&c).unwrap_or(&0))
        .sum()
}
