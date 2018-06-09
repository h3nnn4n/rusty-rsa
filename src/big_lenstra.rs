#![allow(dead_code)]
extern crate gmp_mpfr_sys;
extern crate rand;
extern crate rug;

use self::gmp_mpfr_sys::gmp;
use self::rand::{thread_rng, Rng};
use self::rug::ops::Pow;
use self::rug::rand::RandState;
use self::rug::Integer;
use std::collections::HashMap;

use big_primes;

pub fn modular_inv(a: Integer, b: Integer) -> (Integer, Integer, Integer) {
    if b == 0 {
        return (Integer::from(1), Integer::from(0), a);
    }

    let (q, r) = a.div_rem(b.clone());
    let (x, y, g) = modular_inv(b, r);

    (x.clone(), x - q * y, g)
}

pub fn elliptic_add(
    p: (Integer, Integer, Integer),
    q: (Integer, Integer, Integer),
    a: Integer,
    _b: Integer,
    m: Integer,
) -> (Integer, Integer, Integer) {
    if p.2 == 0 {
        return q;
    }

    if q.2 == 0 {
        return p;
    }

    let num;
    let denom;

    if p.0 == q.0 {
        if (p.1.clone() + q.1) % m.clone() == 0 {
            return (Integer::from(0), Integer::from(1), Integer::from(0));
        }

        num = (3 * p.0.clone().square() + a) % m.clone();
        denom = (2 * p.1.clone()) % m.clone();
    } else {
        num = (q.1 - p.1.clone()) % m.clone();
        denom = (q.0.clone() - p.0.clone()) % m.clone();
    }

    let (inv, _, g) = modular_inv(denom.clone(), m.clone());

    if g > 1 {
        return (Integer::from(0), Integer::from(0), denom.clone());
    }

    let z = (Integer::from(num.clone().square() * inv.clone().square()) - p.0.clone() - q.0.clone())
        % m.clone();

    (
        z.clone(),
        (num * inv * (p.0.clone() - z) - p.1.clone()) % m,
        Integer::from(1),
    )
}

pub fn elliptic_mul(
    k_: Integer,
    p_: (Integer, Integer, Integer),
    a: Integer,
    b: Integer,
    m: Integer,
) -> (Integer, Integer, Integer) {
    let mut r = (Integer::from(0), Integer::from(1), Integer::from(0));

    let mut k = k_.clone();
    let mut p = p_.clone();

    while k > 0 {
        if p.2 > 1 {
            return p;
        }

        if k.is_odd() {
            r = elliptic_add(p.clone(), r, a.clone(), b.clone(), m.clone());
        }

        k = Integer::from(&k / &Integer::from(2));
        p = elliptic_add(p.clone(), p.clone(), a.clone(), b.clone(), m.clone());
    }

    r
}

pub fn lenstra(n: Integer, limit: Integer) -> Option<Integer> {
    let mut g = n.clone();
    let mut rng = rand::thread_rng();

    let mut q = (Integer::new(), Integer::new(), Integer::new());
    let mut a = Integer::new();
    let mut b = Integer::new();

    let n_f64 = n.to_f64();

    while g == n {
        q = (
            big_primes::get_number_with_n_bits((n_f64).log(2.0) as i64),
            big_primes::get_number_with_n_bits((n_f64).log(2.0) as i64),
            big_primes::get_number_with_n_bits((n_f64).log(2.0) as i64),
        );
        a = big_primes::get_number_with_n_bits((n_f64).log(2.0) as i64);

        b = (q.1.clone().pow(2) - q.0.clone().pow(3) - a.clone() * q.0.clone()) % n.clone();
        g = Integer::from(4 * a.clone().pow(3) + 27 * b.clone().pow(2)).gcd(&n);
    }

    if g > 1 {
        return Some(g);
    }

    let mut pp = Integer::from(2);
    let mut p = Integer::from(0);

    let mut i = Integer::new();
    let one = Integer::from(1);

    while i < limit {
        i = Integer::from(&i + &one);
        p = p.next_prime();
        pp = p.clone();

        while pp < limit {
            q = elliptic_mul(p.clone(), q.clone(), a.clone(), b.clone(), n.clone());

            if q.2 > 1 {
                return Some(q.2.gcd(&n));
            }

            pp *= p.clone();
        }
    }

    None
}

pub fn test_lenstra() -> (Integer, Integer) {
    let n = Integer::from(1271);

    let mut p = lenstra(n.clone(), Integer::from(1000));

    while match p {
        None => true,
        _ => false,
    } {
        p = lenstra(n.clone(), Integer::from(1000));
    }

    (n.clone(), p.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lenstra() {
        for _ in 0..10 {
            loop {
                let p = big_primes::get_prime_with_n_bits(15);
                let q = big_primes::get_prime_with_n_bits(10);

                let n = Integer::from(&p * &q);

                let f = super::lenstra(n.clone(), Integer::from(2000));

                let found = match f {
                    Some(_) => true,
                    None => false,
                };

                if found {
                    let w = f.unwrap();

                    if w > 1 {
                        assert!(
                            w.clone() == p.clone() || w.clone() == q.clone(),
                            "{:?} {:?} {:?} {:?}",
                            w,
                            p,
                            q,
                            n
                        );

                        break;
                    }
                }
            }
        }
    }
}
