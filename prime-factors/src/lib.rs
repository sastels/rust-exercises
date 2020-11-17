pub fn factors(n: u64) -> Vec<u64> {
    let mut all_factors: Vec<u64> = Vec::new();
    let mut factor = 2;
    while factor <= n {
        if n % factor == 0 {
            all_factors.push(factor);
            all_factors.extend(factors(n / factor).iter());
            break;
        } else {
            factor += 1
        }
    }
    all_factors
}
