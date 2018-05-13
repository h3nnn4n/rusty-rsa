#![allow(unused_imports)]
extern crate rand;
extern crate rug;
use self::rand::Rng;
use self::rug::Integer;
use big_primes;
use primes;

#[cfg(test)]
mod tests {
    use super::*;

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
            let n = rng.gen_range(2, 10000);
            assert_eq!(big_primes::count_primes(n), primes::count_primes(n));
        }
    }
}
