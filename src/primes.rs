#![allow(dead_code)]

extern crate rand;
use self::rand::Rng;

fn ninja_factor(n: i64) -> (i64, i64) {
    let mut s = 0;
    let mut d = n - 1;

    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    assert!(2_i64.pow(s as u32) * d == n - 1);
    (s, d)
}

fn power(a: i64, n: i64, m: i64) -> i64 {
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

fn miller_rabin(n: i64, s: i64, d: i64, a: i64) -> bool {
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
