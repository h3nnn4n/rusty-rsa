mod big_primes;
mod primes;

fn main() {
    let n = "2305843009213693951";

    big_primes::is_prime_str(n.to_string(), 10);
    big_primes::is_prime_str("53".to_string(), 10);
}
