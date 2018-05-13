mod big_primes;
mod primes;
mod rsa;
mod tests;

fn main() {
    let (private, public) = rsa::get_key(10);
}
