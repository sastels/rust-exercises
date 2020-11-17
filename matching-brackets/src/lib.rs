pub fn is_open_bracket(c: char) -> bool {
    String::from("[({").contains(c)
}

pub fn is_close_bracket(c: char) -> bool {
    String::from("])}").contains(c)
}

pub fn is_bracket(c: char) -> bool {
    is_open_bracket(c) || is_close_bracket(c)
}

fn same_type(c1: char, c2: char) -> bool {
    let b1 = String::from("[]");
    let b2 = String::from("()");
    let b3 = String::from("{}");
    b1.contains(c1) && b1.contains(c2)
        || b2.contains(c1) && b2.contains(c2)
        || b3.contains(c1) && b3.contains(c2)
}

pub fn get_match(s: &str) -> Option<(char, char, usize, usize)> {
    let mut first_bracket: Option<char> = None;
    let mut last_bracket: Option<char> = None;
    let mut first_bracket_index = 0;
    let mut last_bracket_index = 0;
    let mut num_open = 0;
    let mut num_closed = 0;

    for (i, c) in s.chars().enumerate() {
        if is_open_bracket(c) {
            num_open += 1;
            if first_bracket == None {
                first_bracket = Some(c);
                first_bracket_index = i;
            }
        }
        if is_close_bracket(c) {
            num_closed += 1;
            if num_open == num_closed && last_bracket == None {
                last_bracket = Some(c);
                last_bracket_index = i;
            }
        }
    }

    if first_bracket == None || last_bracket == None {
        None
    } else {
        Some((
            first_bracket.unwrap(),
            last_bracket.unwrap(),
            first_bracket_index,
            last_bracket_index,
        ))
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut s = String::from(string);

    while !s.is_empty() {
        let matching = get_match(&s);
        match matching {
            None => return s.chars().filter(|c| is_bracket(*c)).count() == 0,
            Some((c1, c2, i1, i2)) => {
                if !same_type(c1, c2) {
                    return false;
                }
                s = s
                    .chars()
                    .enumerate()
                    .filter(|(i, _c)| *i != i1 && *i != i2)
                    .map(|(_i, c)| c)
                    .collect();
            }
        }
    }
    true
}
