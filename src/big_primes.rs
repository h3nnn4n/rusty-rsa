#![allow(dead_code)]
extern crate rand;
extern crate rug;
use self::rand::Rng;
use self::rug::ops::Pow;
#[allow(unused_imports)]
use self::rug::rand::RandState;
use self::rug::Integer;

pub fn ninja_factor(n: Integer) -> (Integer, Integer) {
    let mut s = Integer::from(0);
    let mut d: Integer = n.clone() - 1;

    loop {
        if Integer::from(&d % 2) != 0 {
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
    let mut nn = n;
    let mut power = a;
    let mut result = Integer::from(1);

    loop {
        if nn.clone() <= 0 {
            break;
        }

        if Integer::from(&nn % 2) == 1 {
            result = Integer::from(Integer::from(&result * &power) % &m);
        }

        power = Integer::from(Integer::from(&power * &power) % &m);

        nn = Integer::from(&nn >> 1);
    }

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
    if (Integer::from(&n % 2) == 0 && Integer::from(&n) != 2) || Integer::from(&n) < 2 {
        return false;
    }
    if Integer::from(&n) <= 3 {
        return true;
    }

    let (s, d) = ninja_factor(n.clone());
    let mut rng = rand::thread_rng();
    //let rand = RandState::new();

    for _ in 0..k {
        let a = Integer::from(rng.gen_range(2, n.to_i64().unwrap()));
        //let a = 2 + (rand.bits(32) % (Integer::from(&n - 4)));
        if !miller_rabin(n.clone(), s.clone(), d.clone(), a) {
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
}
