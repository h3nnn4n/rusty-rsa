mod big_primes;
mod primes;

fn main() {
    let n = "2305843009213693951";

    big_primes::is_prime_str(n.to_string(), 10);
    big_primes::is_prime_str("53".to_string(), 10);

    assert!(big_primes::count_primes(10) == 4);
    assert!(big_primes::count_primes(100) == 25);
    assert!(big_primes::count_primes(1000) == 168);
    assert!(big_primes::count_primes(10000) == 1229);

    assert!(primes::count_primes(10) == 4);
    assert!(primes::count_primes(100) == 25);
    assert!(primes::count_primes(1000) == 168);
    assert!(primes::count_primes(10000) == 1229);
}
