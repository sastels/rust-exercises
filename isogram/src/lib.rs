pub fn check(candidate: &str) -> bool {
    use std::collections::HashSet;

    let candidate = candidate.replace(' ', "").replace('-', "");
    let mut letters = HashSet::new();
    let mut is_isogram = true;

    candidate.chars().for_each(|c| {
        let c = c.to_lowercase().next().unwrap();
        if letters.contains(&c) {
            is_isogram = false;
        }
        letters.insert(c);
    });

    is_isogram
}
