pub fn encode(source: &str) -> String {
    if source.is_empty() {
        return "".to_string();
    }

    let mut encoded = "".to_string();
    let mut prev_char = source.chars().next().unwrap();
    let mut char_count = 1;

    for c in source[1..].chars() {
        if c == prev_char {
            char_count += 1;
        } else {
            if char_count == 1 {
                encoded = format!("{}{}", encoded, prev_char);
            } else {
                encoded = format!("{}{}{}", encoded, char_count, prev_char);
            }
            prev_char = c;
            char_count = 1;
        }
    }
    if char_count == 1 {
        encoded = format!("{}{}", encoded, prev_char);
    } else {
        encoded = format!("{}{}{}", encoded, char_count, prev_char);
    }
    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = "".to_string();
    let mut counter = "".to_string();

    for c in source.chars() {
        if c.is_ascii_digit() {
            counter = format!("{}{}", counter, c);
        } else {
            if counter.is_empty() {
                counter = "1".to_string();
            }
            decoded = format!(
                "{}{}",
                decoded,
                c.to_string().repeat(counter.parse().unwrap())
            );
            counter = "".to_string();
        }
    }
    decoded
}
