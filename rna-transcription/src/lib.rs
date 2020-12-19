#[derive(Debug, PartialEq)]
pub struct Dna {
    strand: Vec<char>,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    strand: Vec<char>,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let strand: Vec<char> = dna.chars().collect();
        let error = strand
            .iter()
            .enumerate()
            .filter(|(_, c)| !['A', 'C', 'G', 'T'].contains(c))
            .map(|(n, _)| n)
            .next();
        match error {
            None => Ok(Dna { strand }),
            Some(n) => Err(n),
        }
    }

    pub fn into_rna(self) -> Rna {
        let strand: String = self
            .strand
            .iter()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => panic!(),
            })
            .collect();
        Rna::new(&strand).unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let strand: Vec<char> = rna.chars().collect();
        let error = strand
            .iter()
            .enumerate()
            .filter(|(_, c)| !['A', 'C', 'G', 'U'].contains(c))
            .map(|(n, _)| n)
            .next();
        match error {
            None => Ok(Rna { strand }),
            Some(n) => Err(n),
        }
    }
}
