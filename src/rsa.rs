extern crate rug;
use self::rug::ops::Pow;
use self::rug::rand::RandState;
use self::rug::Integer;

use big_primes;

fn mod_inv(a: Integer, module: Integer) -> Integer {
    let mut mn = (module.clone(), a.clone());
    let mut xy = (Integer::from(0), Integer::from(1));

    while mn.1 != 0 {
        xy = (
            xy.1.clone(),
            xy.0.clone() - Integer::from(&mn.0 / &mn.1) * xy.1.clone(),
        );
        mn = (mn.1.clone(), Integer::from(&mn.0 % &mn.1));
    }

    while xy.0.clone() < 0 {
        xy.0 = Integer::from(&xy.0 + &module);
    }

    xy.0
}

pub fn get_pq(n_bits: i64) -> (i64, i64) {
    let p = big_primes::get_prime_with_n_bits(n_bits / 2);
    let q = big_primes::get_prime_with_n_bits(n_bits / 2);
    let n = Integer::from(&p * &q);
    let tot = Integer::from(Integer::from(&p - 1) * Integer::from(&q - 1));
    let e = big_primes::get_prime_with_n_bits(8);
    let d = mod_inv(e.clone(), tot.clone());
    (10, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_inv() {
        assert_eq!(super::mod_inv(Integer::from(7), Integer::from(40)), 23);
        assert_eq!(super::mod_inv(Integer::from(42), Integer::from(2017)), 1969);
    }
}
