mod big_primes;
mod primes;
mod rsa;
mod tests;

fn main() {
    let n_bits = 32;
    let (private, public) = rsa::get_key(n_bits);

    rsa::encrypt_file(
        "test".to_string(),
        "test.enc".to_string(),
        public.clone(),
        n_bits,
    );

    println!();

    rsa::decrypt_file(
        "test.enc".to_string(),
        "test.dec".to_string(),
        private.clone(),
        n_bits,
    );

    println!();
}
