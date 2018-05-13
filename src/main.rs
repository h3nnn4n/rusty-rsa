mod primes;

fn main() {
    assert!(primes::count_primes(10) == 4);
    assert!(primes::count_primes(100) == 25);
    assert!(primes::count_primes(1000) == 168);
    assert!(primes::count_primes(10000) == 1229);
}
