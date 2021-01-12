/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let multipliers = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    let isbn = isbn.replace('-', "");

    if isbn.len() != 10 {
        return false;
    }
    let mut check_digit = isbn.chars().last().unwrap();
    check_digit.make_ascii_uppercase();
    let rest = &isbn[..9];
    if (!check_digit.is_ascii_digit() && check_digit != 'X')
        || !rest.chars().all(|x| x.is_ascii_digit())
    {
        return false;
    }

    let isbn_sum: usize = isbn
        .chars()
        .zip(multipliers)
        .map(|(c, m)| {
            if c == 'X' {
                10 * m
            } else {
                c.to_string().parse::<usize>().unwrap() * m
            }
        })
        .sum();

    isbn_sum % 11 == 0
}
