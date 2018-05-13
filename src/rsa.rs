extern crate rug;
use self::rug::ops::Pow;
use self::rug::rand::RandState;
use self::rug::Integer;

use big_primes;

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        return (b, 0, 1);
    } else {
        let (g, x, y) = egcd(b % a, a);

        return (g, y - (b / a) * x, x);
    }
}

pub fn get_pq(n_bits: i64) -> (i64, i64) {
    let p = big_primes::get_prime_with_n_bits(n_bits / 2);
    let q = big_primes::get_prime_with_n_bits(n_bits / 2);
    let n = Integer::from(&p * &q);
    let tot = Integer::from(Integer::from(&p - 1) * Integer::from(&q - 1));
    let e = big_primes::get_prime_with_n_bits(8);
    (10, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn egcd() {
        assert_eq!(super::egcd(102, 38).0, 2);
        assert_eq!(super::egcd(42823, 6409).0, 17);

        assert_eq!(super::egcd(1914, 899).1, 8);
        assert_eq!(super::egcd(1914, 899).2, -17);

        assert_eq!(super::egcd(1432, 123211).1, -22973);
        assert_eq!(super::egcd(1432, 123211).2, 267);
    }
}
