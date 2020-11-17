const SIZE: usize = 1000000;

pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![];
    let mut sieve: [bool; SIZE] = [true; SIZE];
    for i in 2..SIZE {
        if sieve[i] {
            primes.push(i as u32);
            let mut i_multiple = i * 2;
            while i_multiple < SIZE {
                sieve[i_multiple] = false;
                i_multiple += i;
            }
        }
    }
    return primes[n as usize];
}
