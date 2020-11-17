pub fn reply(message: &str) -> &str {
    println!("message <{}>", message);

    let mut s = String::from(message);
    s.retain(|c| !c.is_whitespace());
    let mut question = false;
    let mut yell = false;
    let mut no_letters = false;

    if s.chars().filter(|c| c.is_alphabetic()).count() == 0 {
        no_letters = true
    }
    if s.ends_with('?') {
        question = true;
    }
    if s.eq(&s.to_ascii_uppercase()) && !no_letters {
        yell = true
    }

    if no_letters && !s.is_empty() && !question {
        "Whatever."
    } else if question && yell {
        "Calm down, I know what I'm doing!"
    } else if question {
        "Sure."
    } else if s.chars().all(char::is_whitespace) {
        "Fine. Be that way!"
    } else if yell {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
