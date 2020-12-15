use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !['A', 'C', 'G', 'T'].contains(&nucleotide) {
        return Err(nucleotide);
    }
    let errors: Vec<char> = dna
        .chars()
        .filter(|c| !['A', 'C', 'G', 'T'].contains(&c))
        .collect();
    if !errors.is_empty() {
        return Err(errors[0]);
    }
    Ok(dna.chars().filter(|c| *c == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = HashMap::new();
    counts.insert('A', count('A', dna)?);
    counts.insert('C', count('C', dna)?);
    counts.insert('T', count('T', dna)?);
    counts.insert('G', count('G', dna)?);
    Ok(counts)
}
