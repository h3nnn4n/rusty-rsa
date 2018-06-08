#![allow(dead_code)]
extern crate rand;
use self::rand::Rng;

pub fn prime_factorization_brute_force(x: i64) -> Vec<i64> {
    let mut factors: Vec<i64> = Vec::new();
    let mut p = 3;
    let stop = (x as f64).sqrt() as i64;

    if x % 2 == 0 {
        factors.push(2);
    }

    loop {
        if x % p == 0 {
            factors.push(p);
            factors.push(x / p);
        }

        p = &p + 2;

        if p > stop {
            break;
        }
    }

    factors.push(1);
    factors.push(x);
    factors.sort();

    factors
}

pub fn ninja_factor(n: i64) -> (i64, i64) {
    let mut s = 0;
    let mut d = n - 1;

    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    assert_eq!(2_i64.pow(s as u32) * d, n - 1);
    (s, d)
}

pub fn power(a: i64, n: i64, m: i64) -> i64 {
    let mut nn = n;
    let mut power = a;
    let mut result = 1;

    while nn > 0 {
        if nn % 2 == 1 {
            result = (result * power) % m;
        }

        power = (power * power) % m;
        nn >>= 1;
    }

    result
}

pub fn miller_rabin(n: i64, s: i64, d: i64, a: i64) -> bool {
    let mut x = power(a, d, n);
    let mut y = 0;

    for _r in 0..s {
        y = (x * x) % n;
        if y == 1 && x != 1 && x != n - 1 {
            return false;
        }
        x = y;
    }

    return if y == 1 { true } else { false };
}

pub fn is_prime(n: i64, k: i64) -> bool {
    if (n % 2 == 0 && n != 2) || (n < 2) {
        return false;
    }
    if n <= 3 {
        return true;
    }

    let mut rng = rand::thread_rng();
    let (s, d) = ninja_factor(n);

    for _ in 0..k {
        let a = rng.gen_range(2, n);
        if !miller_rabin(n, s, d, a) {
            return false;
        }
    }

    true
}

pub fn count_primes(upper: i64) -> i64 {
    let mut total = 0;
    let k = 10;

    for n in 2..upper {
        if is_prime(n, k) {
            total += 1
        }
    }

    total
}

pub fn get_prime_with_n_bits(n_bits: i64) -> i64 {
    let mut rng = rand::thread_rng();

    let upper = (2_f64.powi(n_bits as i32 + 1_i32) - 1.0) as i64;
    let lower = 2_f64.powi(n_bits as i32) as i64;

    loop {
        let n = rng.gen_range(lower, upper);

        if is_prime(n, 20) {
            return n;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primes_less_than_10() {
        assert!(count_primes(10) == 4);
    }

    #[test]
    fn primes_less_than_100() {
        assert!(count_primes(100) == 25);
    }

    #[test]
    fn primes_less_than_1000() {
        assert!(count_primes(1000) == 168);
    }

    #[test]
    fn primes_less_than_10000() {
        assert!(count_primes(10000) == 1229);
    }

    #[test]
    fn brute_force_factorization() {
        for _ in 0..10 {
            let n = get_prime_with_n_bits(4);
            let m = get_prime_with_n_bits(4);

            let p = m * n;

            let mut q = vec![1, m, n, p];

            q.sort();

            assert_eq!(prime_factorization_brute_force(p), q);
        }
    }
}
