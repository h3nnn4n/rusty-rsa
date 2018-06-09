#![allow(unused_imports)]
extern crate rand;
extern crate rug;

use self::rand::Rng;
use self::rug::Integer;
use big_lenstra;
use big_primes;
use lenstra;
use primes;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modular_inv() {
        fn is_modular_inv_eq(a: i64, b: i64) -> bool {
            let p = lenstra::modular_inv(a, b);
            let q = big_lenstra::modular_inv(Integer::from(a), Integer::from(b));

            p.0 == q.0 && p.1 == q.1
        }

        let mut rng = rand::thread_rng();

        for _ in 0..50 {
            while {
                let p = rng.gen_range(100, 1000);
                let q = rng.gen_range(100, 1000);

                let mut n = Integer::from(p);

                let has_inverse = match n.invert(&Integer::from(q)) {
                    Ok(inverse) => true,
                    Err(_) => false,
                };

                if has_inverse {
                    assert!(is_modular_inv_eq(p, q));
                }

                !has_inverse
            } {}
        }
    }

    #[test]
    fn elliptic_add() {
        let mut rng = rand::thread_rng();

        for _ in 0..50 {
            let n = rng.gen_range(100, 1000);
            let p = (rng.gen_range(0, n - 1), rng.gen_range(0, n - 1), 1);
            let q = (rng.gen_range(0, n - 1), rng.gen_range(0, n - 1), 1);
            let p_ = (Integer::from(p.0), Integer::from(p.1), Integer::from(p.2));
            let q_ = (Integer::from(q.0), Integer::from(q.1), Integer::from(q.2));
            let a = rng.gen_range(0, n - 1);

            let x = lenstra::elliptic_add(p, q, a, 0, n);
            let y = big_lenstra::elliptic_add(
                p_,
                q_,
                Integer::from(a),
                Integer::from(0),
                Integer::from(n),
            );
        }
    }

    #[test]
    fn power() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let a = rng.gen_range(2, 100);
            let n = rng.gen_range(2, 100);
            let m = rng.gen_range(2, 100);

            let result1 = primes::power(a, n, m);
            let result2 = big_primes::power(Integer::from(a), Integer::from(n), Integer::from(m));
            assert_eq!(result1, result2);
        }
    }

    #[test]
    fn ninja_factor() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let n = rng.gen_range(2, 10000);

            let (a, b) = primes::ninja_factor(n);
            let (a2, b2) = big_primes::ninja_factor(Integer::from(n));

            assert_eq!(a, a2);
            assert_eq!(b, b2);
        }
    }

    #[test]
    fn miller_rabin() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let n = rng.gen_range(2, 10000);
            let a = rng.gen_range(2, n);

            let (s, d) = primes::ninja_factor(n);

            let result1 = primes::miller_rabin(n, s, d, a);
            let result2 = big_primes::miller_rabin(
                Integer::from(n),
                Integer::from(s),
                Integer::from(d),
                Integer::from(a),
            );

            assert_eq!(result1, result2);
        }
    }

    #[test]
    fn count_primes() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let n = rng.gen_range(2, 1000);
            assert_eq!(big_primes::count_primes(n), primes::count_primes(n));
        }
    }
}
