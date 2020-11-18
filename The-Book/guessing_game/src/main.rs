use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1, 101);

    println!("secret: {}", secret);

    loop {
        println!("Please enter your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("couldn't read stdin");

        let guess: u32 = match guess.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                println!("Numbers only");
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("Right on!");
                break;
            }
            Ordering::Greater => println!("too big"),
        }
    }
}
