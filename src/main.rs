use std::env;

mod big_primes;
mod primes;
mod rsa;
mod tests;

fn rsa_magic(n_bits: i64) {
    let (private, public) = rsa::get_key(n_bits);
    //let (private, public) = (
    //(Integer::from(11483), Integer::from(26123)),
    //(Integer::from(33347), Integer::from(26123)),
    //);

    println!("{:?} {:?}", private, public);

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

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let n_bits_try = args[1].parse();

    let n_bits = match n_bits_try {
        Ok(n) => n,
        Err(_) => 24,
    };

    rsa_magic(n_bits);
}
