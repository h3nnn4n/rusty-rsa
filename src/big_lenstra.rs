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
