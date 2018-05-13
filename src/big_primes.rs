#![allow(dead_code)]
extern crate rug;
use self::rug::rand::RandState;
use self::rug::Integer;

fn ninja_factor(n: Integer) -> (Integer, Integer) {
    let mut s = Integer::from(0);
    let mut d: Integer = n - 1;

    loop {
        if Integer::from(&d % 2) != 0 {
            break;
        }

        d = Integer::from(&d / 2);
        s = Integer::from(&s + 1);
    }

    (s, d)
}

fn power(a: Integer, n: Integer, m: Integer) -> Integer {
    let mut nn = n;
    let mut power = a;
    let mut result = Integer::from(1);

    loop {
        if Integer::from(&nn % 2) == 0 {
            break;
        }
        if Integer::from(&nn % 2) == 1 {
            result = Integer::from(Integer::from(&result * &result) % &m);
        }

        power = Integer::from(Integer::from(&power * &power) % &m);

        nn = Integer::from(&nn >> 1);
    }

    result
}

fn miller_rabin(n: Integer, s: Integer, d: Integer, a: Integer) -> bool {
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

    for _ in 0..k {
        let mut rand = RandState::new();
        let a = 2 + (rand.bits(32) % (Integer::from(&n - 4)));
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
            println!("{}", n);
            total += 1
        }
    }

    total
}