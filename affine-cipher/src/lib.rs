use num::Integer;

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

fn encrypt_char(c: char, a: i32, b: i32) -> char {
    if c.is_ascii_digit() {
        c
    } else {
        let mut ascii = c as i32 - ('a' as i32);
        ascii = (a * ascii + b) % 26;
        (ascii + ('a' as i32)) as u8 as char
    }
}

fn decrypt_char(c: char, a: i32, b: i32) -> char {
    let e = i32::extended_gcd(&a, &26);
    let inverse = e.x % 26;

    if c.is_ascii_digit() {
        c
    } else {
        let mut ascii = c as i32 - ('a' as i32);
        ascii = inverse * (ascii - b) % 26;
        if ascii < 0 {
            ascii += 26;
        }
        (ascii + ('a' as i32)) as u8 as char
    }
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if a.gcd(&26) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let ciphertext: String = plaintext
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| encrypt_char(c, a, b))
        .collect();

    let chars: Vec<char> = ciphertext.chars().collect();
    let split = &chars
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>();

    Ok(split.join(" "))
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if a.gcd(&26) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }
    let plaintext: String = ciphertext
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| decrypt_char(c, a, b))
        .collect();

    Ok(plaintext)
}
