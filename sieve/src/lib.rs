use bit_vec::BitVec;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let upper_bound = upper_bound as usize;
    let mut bv = BitVec::from_elem(upper_bound + 1, true);
    if upper_bound < 2 {
        return vec![];
    }
    bv.set(0, false);
    bv.set(1, false);

    let mut n: usize = 2;
    loop {
        if n > upper_bound {
            break;
        }
        if bv[n] {
            for n_mult in ((2 * n)..=upper_bound).step_by(n) {
                bv.set(n_mult, false)
            }
        }
        n += 1; // after 2 could just do odds ¯\_(ツ)_/¯
    }
    bv.into_iter()
        .enumerate()
        .filter(|(_, is_prime)| *is_prime)
        .map(|(n, _)| n as u64)
        .collect()
}
