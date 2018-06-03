#![allow(dead_code)]
extern crate rand;
extern crate rug;
use self::rand::{thread_rng, Rng};
use self::rug::ops::Pow;
use self::rug::rand::RandState;
use self::rug::Integer;

pub fn prime_factorization_brute_force(x: Integer) -> Vec<Integer> {
    let mut factors: Vec<Integer> = Vec::new();
    let mut p = Integer::from(3);
    let stop = Integer::from(x.clone().sqrt());

    if x.clone() % 2 == 0 {
        factors.push(Integer::from(2));
    }

    loop {
        if x.clone() % p.clone() == 0 {
            factors.push(p.clone());
            factors.push(x.clone() / p.clone());
        }

        p = Integer::from(&p + Integer::from(2));

        if p > stop {
            break;
        }
    }

    factors.push(Integer::from(1));
    factors.push(x.clone());
    factors.sort();

    factors
}

pub fn prime_factorization_fermats(n: Integer) -> Vec<Integer> {
    let mut factors: Vec<Integer> = Vec::new();

    //FermatFactor(N): // N should be odd
    assert!(n.is_odd());

    factors.push(Integer::from(1));
    factors.push(n.clone());

    let one = Integer::from(1);

    //a ← ceil(sqrt(N))
    let mut a = Integer::from(n.sqrt_ref());

    //b2 ← a*a - N
    let mut b2 = Integer::from(Integer::from(&a * &a) - &n);

    //while b2 is not a square:
    while !b2.is_perfect_square() {
        //    a ← a + 1    // equivalently: b2 ← b2 + 2*a + 1
        a = Integer::from(&a + &one);
        //    b2 ← a*a - N //               a ← a + 1
        b2 = Integer::from(Integer::from(&a * &a) - &n);
        //endwhile
    }

    //return a - sqrt(b2) // or a + sqrt(b2)
    factors.push(Integer::from(&a - Integer::from(b2.sqrt_ref())));
    factors.push(Integer::from(&a + Integer::from(b2.sqrt_ref())));

    factors.sort();

    factors
}

pub fn prime_factorization_pollard_rho(n: Integer) -> Vec<Integer> {
    let mut factors: Vec<Integer> = Vec::new();

    factors.push(Integer::from(1));
    factors.push(n.clone());

    fn pollard_rho_plus_one_step(n: Integer, k: Integer) -> (Integer, Integer) {
        let mut x = get_number_with_n_bits((n.significant_bits() / 10) as i64);
        let mut y = x.clone();
        let mut d: Integer;
        let one = Integer::from(1);
        let g = |x: Integer| Integer::from(Integer::from(&x * &x) + &k) % n.clone();

        loop {
            x = g(x);
            //x = Integer::from(&x.pow(2) + &k) % n.clone();

            y = g(g(y));
            //y = Integer::from(&y.pow(2) + &k) % n.clone();
            //y = Integer::from(&y.pow(2) + &k) % n.clone();

            d = Integer::from(Integer::from(&x - &y).abs());
            d = d.gcd(&n.clone());

            if d != one {
                break;
            }
        }

        if d == n {
            return (one, n.clone());
        } else {
            return (d.clone(), n.clone() / d.clone());
        }
    }

    let mut i = 1;
    while factors.len() < 4 {
        let x = if i % 2 == 0 {
            Integer::from(i / 2)
        } else {
            get_number_with_n_bits(5_i64)
        };

        let (p, q) = pollard_rho_plus_one_step(n.clone(), x);

        if !factors.contains(&p) {
            factors.push(p.clone());

            if p == q.clone() {
                factors.push(q.clone());
            }
        }

        if !factors.contains(&q) {
            factors.push(q);
        }

        i += 1;
    }

    factors.sort();

    factors
}

pub fn get_number_with_n_bits(n_bits: i64) -> Integer {
    let consonants = b"01";
    let value: Integer;

    loop {
        let mut result = String::new();
        result.push('1');

        for _ in 0..n_bits - 2 {
            result.push(thread_rng().choose(consonants).cloned().unwrap().into());
        }

        result.push('1');

        let valid1 = Integer::parse_radix(result, 2);
        value = Integer::from(valid1.unwrap());

        if value.significant_bits() as i64 == n_bits {
            break;
        } else {
            println!(
                "[WARN] Expected number with {} bits, got {} instead",
                n_bits,
                value.significant_bits()
            );
            panic!();
        }
    }

    value
}

pub fn get_prime_with_n_bits(n_bits: i64) -> Integer {
    loop {
        let n = get_number_with_n_bits(n_bits);
        if is_prime(n.clone(), 20) {
            return n;
        }
    }
}

pub fn ninja_factor(n: Integer) -> (Integer, Integer) {
    let mut s = Integer::from(0);
    let mut d: Integer = n.clone() - 1;

    loop {
        if d.is_odd() {
            break;
        }

        d = Integer::from(&d / 2);
        s = Integer::from(&s + 1);
    }

    let b = Integer::from(2);
    let e: u32 = s.to_u32().unwrap();
    let p = b.pow(e);
    assert_eq!(Integer::from(&p * &d), Integer::from(&n - 1));
    (s, d)
}

pub fn power(a: Integer, n: Integer, m: Integer) -> Integer {
    let n_ = a.clone();
    let e_ = n.clone();
    let m_ = m.clone();

    let check = match n_.pow_mod(&e_, &m_) {
        Ok(check) => check,
        Err(_) => unreachable!(),
    };

    let mut nn = n;
    let mut power = a;
    let mut result = Integer::from(1);

    loop {
        if nn.clone() <= 0 {
            break;
        }

        if nn.is_odd() {
            result = Integer::from(Integer::from(&result * &power) % &m);
        }

        power = Integer::from(Integer::from(&power * &power) % &m);

        nn = Integer::from(&nn >> 1);
    }

    assert_eq!(result, check);

    result
}

pub fn miller_rabin(n: Integer, s: Integer, d: Integer, a: Integer) -> bool {
    let mut x = power(a, d, n.clone());
    let mut y = Integer::from(0);

    let mut _r = s;
    while _r > 0 {
        y = power(x.clone(), Integer::from(2), n.clone());
        if y == 1 && x != 1 && x != Integer::from(&n - 1) {
            return false;
        }
        x = Integer::from(&y);
        _r = Integer::from(&_r - 1);
    }

    return if y == 1 { true } else { false };
}

pub fn is_prime(n: Integer, k: i64) -> bool {
    if (n.is_even() && Integer::from(&n) != 2) || Integer::from(&n) < 2 {
        return false;
    }
    if Integer::from(&n) <= 3 {
        return true;
    }

    let (s, d) = ninja_factor(n.clone());
    let mut rand = RandState::new();

    for _ in 0..k {
        let a2: Integer = 2 + (rand.bits(32) % (Integer::from(&n - 4)));
        if !miller_rabin(n.clone(), s.clone(), d.clone(), a2.clone()) {
            return false;
        }
    }

    true
}

pub fn is_prime_str(s: String, k: i64) -> bool {
    let nn = s.parse::<Integer>().unwrap();
    return is_prime(nn, k);
}

pub fn count_primes(upper: i64) -> i64 {
    let mut total = 0;
    let k = 10;

    for n in 2..upper {
        if is_prime(Integer::from(n), k) {
            total += 1
        }
    }

    total
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
    fn pollard_rho_factorization() {
        for _ in 0..10 {
            let m = get_prime_with_n_bits(10);
            let n = get_prime_with_n_bits(15);

            let p = Integer::from(&m * &n);

            let mut q = vec![Integer::from(1), m, n, p.clone()];

            q.sort();

            assert_eq!(prime_factorization_pollard_rho(p), q);
        }
    }

    #[test]
    fn fermats_factorization() {
        for _ in 0..10 {
            let m = get_prime_with_n_bits(20);
            let n = get_prime_with_n_bits(20);

            let p = Integer::from(&m * &n);

            let mut q = vec![Integer::from(1), m, n, p.clone()];

            q.sort();

            assert_eq!(prime_factorization_fermats(p), q);
        }
    }

    #[test]
    fn brute_force_factorization() {
        for _ in 0..10 {
            let m = get_prime_with_n_bits(4);
            let n = get_prime_with_n_bits(4);

            let p = Integer::from(&m * &n);

            let mut q = vec![Integer::from(1), m, n, p.clone()];

            q.sort();

            assert_eq!(prime_factorization_brute_force(p), q);
        }
    }
}
