use ac_lib::math::{gcd, generate_primes, is_prime, lcm, nth_prime, ModInt};

#[test]
fn test_gcd() {
    assert_eq!(gcd(48, 18), 6);
    assert_eq!(gcd(101, 10), 1);
    assert_eq!(gcd(12, 8), 4);
    assert_eq!(gcd(0, 5), 5);
    assert_eq!(gcd(5, 0), 5);
}

#[test]
fn test_lcm() {
    assert_eq!(lcm(4, 6), 12);
    assert_eq!(lcm(3, 5), 15);
    assert_eq!(lcm(12, 8), 24);
}

#[test]
fn test_is_prime() {
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(!is_prime(4));
    assert!(is_prime(5));
    assert!(is_prime(17));
    assert!(is_prime(97));
    assert!(!is_prime(100));
}

#[test]
fn test_generate_primes() {
    let primes = generate_primes(20);
    assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19]);

    let primes_10 = generate_primes(10);
    assert_eq!(primes_10, vec![2, 3, 5, 7]);
}

#[test]
fn test_nth_prime() {
    assert_eq!(nth_prime(1), Some(2));
    assert_eq!(nth_prime(2), Some(3));
    assert_eq!(nth_prime(3), Some(5));
    assert_eq!(nth_prime(10), Some(29));
}

#[test]
fn test_modint_new() {
    let m = ModInt::new(5, 7);
    assert_eq!(m.value(), 5);

    let m2 = ModInt::new(-3, 7);
    assert_eq!(m2.value(), 4);

    let m3 = ModInt::new(10, 7);
    assert_eq!(m3.value(), 3);
}

#[test]
fn test_modint_add() {
    let a = ModInt::new(5, 7);
    let b = ModInt::new(3, 7);
    let c = a.add(&b);
    assert_eq!(c.value(), 1);
}

#[test]
fn test_modint_sub() {
    let a = ModInt::new(5, 7);
    let b = ModInt::new(3, 7);
    let c = a.sub(&b);
    assert_eq!(c.value(), 2);

    let d = ModInt::new(2, 7);
    let e = ModInt::new(5, 7);
    let f = d.sub(&e);
    assert_eq!(f.value(), 4);
}

#[test]
fn test_modint_mul() {
    let a = ModInt::new(5, 7);
    let b = ModInt::new(3, 7);
    let c = a.mul(&b);
    assert_eq!(c.value(), 1);
}

#[test]
fn test_modint_pow() {
    let a = ModInt::new(2, 1000000007);
    let b = a.pow(10);
    assert_eq!(b.value(), 1024);

    let c = ModInt::new(3, 7);
    let d = c.pow(3);
    assert_eq!(d.value(), 6);
}

#[test]
fn test_modint_clone() {
    let a = ModInt::new(5, 7);
    let b = a.clone();
    assert_eq!(a.value(), b.value());
}

#[test]
#[should_panic(expected = "Moduli must be the same")]
fn test_modint_different_modulus() {
    let a = ModInt::new(5, 7);
    let b = ModInt::new(3, 11);
    let _ = a.add(&b); // Should panic
}

#[test]
fn test_gcd_edge_cases() {
    assert_eq!(gcd(1, 1), 1);
    assert_eq!(gcd(100, 100), 100);
    assert_eq!(gcd(1000000, 1), 1);
}

#[test]
fn test_lcm_edge_cases() {
    assert_eq!(lcm(1, 1), 1);
    assert_eq!(lcm(1, 100), 100);
    assert_eq!(lcm(7, 13), 91);
}

#[test]
fn test_is_prime_large() {
    assert!(is_prime(1009));
    assert!(is_prime(10007));
    assert!(!is_prime(10000));
    assert!(!is_prime(10001));
}

#[test]
fn test_generate_primes_edge_cases() {
    let primes = generate_primes(2);
    assert_eq!(primes, vec![2]);

    let primes_1 = generate_primes(1);
    assert_eq!(primes_1, Vec::<u64>::new());
}

#[test]
fn test_nth_prime_edge_cases() {
    assert_eq!(nth_prime(1), Some(2));
    assert_eq!(nth_prime(25), Some(97));
    assert_eq!(nth_prime(100), Some(541));
}

#[test]
fn test_modint_large_numbers() {
    let mod_val = 1000000007;
    let a = ModInt::new(999999999, mod_val);
    let b = ModInt::new(999999999, mod_val);
    let c = a.add(&b);
    assert_eq!(c.value(), 999999991);
}
#[test]
fn test_modint_pow_zero() {
    let a = ModInt::new(5, 7);
    let b = a.pow(0);
    assert_eq!(b.value(), 1);
}

#[test]
fn test_modint_pow_one() {
    let a = ModInt::new(5, 7);
    let b = a.pow(1);
    assert_eq!(b.value(), 5);
}

#[test]
fn test_modint_chain_operations() {
    let a = ModInt::new(2, 7);
    let b = ModInt::new(3, 7);
    let c = ModInt::new(4, 7);

    let result = a.add(&b).mul(&c);
    assert_eq!(result.value(), 6);
}
