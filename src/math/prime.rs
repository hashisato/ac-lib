pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn generate_primes(limit: u64) -> Vec<u64> {
    (2..=limit).filter(|&n| is_prime(n)).collect()
}

pub fn nth_prime(n: usize) -> Option<u64> {
    let mut count = 0;
    let mut candidate = 1;

    while count < n {
        candidate += 1;
        if is_prime(candidate) {
            count += 1;
        }
    }
    Some(candidate)
}
