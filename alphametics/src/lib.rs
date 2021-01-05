use itertools::Itertools;
use std::collections::HashMap;

fn check_eqn(input: &str, vals: &HashMap<char, u8>) -> bool {
    let mut equation = input.to_string().replace(' ', "");
    for (key, val) in vals {
        equation = equation.replace(*key, &val.to_string());
    }
    let sum = equation.split("==").next().unwrap();
    let answer: String = equation.split("==").last().unwrap().to_string();
    if answer.starts_with('0') {
        return false;
    }
    let answer: usize = answer.parse().unwrap();
    let summands: Vec<String> = sum.split('+').map(|s| s.to_string()).collect();
    if summands.iter().any(|s| s.starts_with('0')) {
        return false;
    }
    let sum: usize = summands.iter().map(|x| x.parse::<usize>().unwrap()).sum();

    sum == answer
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let letters: Vec<_> = input
        .chars()
        .filter(|c| c.is_alphabetic())
        .unique()
        .collect();

    let perms: Vec<_> = (0..10).permutations(letters.len()).collect();

    let solutions: Vec<HashMap<_, _>> = perms
        .iter()
        .map(|v| {
            let solns: HashMap<char, u8> = letters.iter().cloned().zip(v.iter().cloned()).collect();
            return solns;
        })
        .collect();

    let solutions: Vec<_> = solutions
        .iter()
        .filter(|solns| check_eqn(input, solns))
        .collect();

    if !solutions.is_empty() {
        return Some(solutions[0].clone());
    }
    return None;
}
