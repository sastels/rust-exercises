use regex::Regex;

fn upper(c: char) -> char {
    c.to_uppercase().collect::<Vec<_>>()[0]
}

fn abbreviate_word(word: &str) -> String {
    println!("word {}", word);
    let mut word = word.to_string();

    // if all caps, take first letter
    if word.chars().all(|c| c == upper(c)) {
        word = word.chars().next().unwrap().to_string();
    }

    // capitalize
    let word: String = word
        .chars()
        .enumerate()
        .map(|(n, c)| if n == 0 { upper(c) } else { c })
        .collect();

    // take only caps
    let word: String = word.chars().filter(|c| *c == upper(*c)).collect();
    word
}

pub fn abbreviate(phrase: &str) -> String {
    // delete apostrophes
    let phrase = phrase.to_string().replace("'", "");

    // replace punctuation with space
    let re = Regex::new(r"[^\w\s]").unwrap();
    let phrase = re.replace_all(&phrase, " ").to_string().replace("_", " ");

    // split into words
    let words: Vec<String> = phrase
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    // transform into acronyms
    let words: Vec<String> = words.iter().map(|w| abbreviate_word(w)).collect();

    words.join("")
}
