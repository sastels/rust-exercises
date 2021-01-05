use itertools::Itertools;
use std::collections::HashMap;

fn check_eqn(input: &str, vals: &HashMap<char, u8>) -> bool {
    let mut equation = input.to_string().replace(' ', "");
    for (key, val) in vals {
        equation = equation.replace(*key, &val.to_string());
    }
    let sum = equation.split("==").next().unwrap();
    let answer: usize = equation.split("==").last().unwrap().parse().unwrap();
    let sum: usize = sum.split('+').map(|x| x.parse::<usize>().unwrap()).sum();
    println!("{:?}  sum: {} answer {}", &vals, sum, answer);

    sum == answer
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let letters: Vec<_> = input
        .chars()
        .filter(|c| c.is_alphabetic())
        .unique()
        .collect();

    println!("letters: {:?}", letters);
    let perms: Vec<_> = (0..10).permutations(letters.len()).collect();

    let solutions = perms.iter().map(|v| {
        let solns: HashMap<char, u8> = letters.iter().cloned().zip(v.iter().cloned()).collect();
        return solns;
    });

    let solutions: Vec<_> = solutions.filter(|solns| check_eqn(input, solns)).collect();

    if !solutions.is_empty() {
        return Some(solutions[0].clone());
    }
    return None;
}
