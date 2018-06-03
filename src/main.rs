extern crate clap;

use clap::{App, Arg};
use std::time::Instant;
mod big_primes;
mod primes;
mod rsa;
mod tests;

fn main() {
    let matches = App::new("Rusty Rsa")
        .version("0.1")
        .author("Renan S Silva <uber.renan@gmail.com>")
        .about("Does awesome mathy crypto things")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required_unless_one(&["GEN", "BRUTE", "POLLARD", "POLLARD_RAW", "FERMAT", "FERMAT_RAW"])
                .index(1),
        )
        .arg(
            Arg::with_name("ENCRYPT")
                .short("-e")
                .long("--encrypt")
                .conflicts_with("DECRYPT")
                .help("Encrypts a file using the specified key"),
        )
        .arg(
            Arg::with_name("DECRYPT")
                .short("-d")
                .long("--decrypt")
                .conflicts_with("ENCRYPT")
                .help("Decrypts a file using the specified key"),
        )
        .arg(
            Arg::with_name("KEYSIZE")
                .short("-b")
                .long("--keysize")
                .takes_value(true)
                .help("Sets the keysize. Defaults to 32"),
        )
        .arg(
            Arg::with_name("GEN")
                .short("-g")
                .long("--generate_key")
                .requires("KEY")
                .conflicts_with_all(&["ENCRYPT", "DECRYPT"])
                .help("Generates a new key pair"),
        )
        .arg(
            Arg::with_name("KEY")
                .short("-k")
                .long("--key")
                .takes_value(true)
                .required_unless("GEN")
                .help("Sets the key to be used."),
        )
        .arg(
            Arg::with_name("BRUTE")
                .long("--bruteforce")
                .conflicts_with_all(&["ENCRYPT", "DECRYPT", "GEN", "POLLARD"])
                .help("Breaks a public key using simple and dumb factorization"),
        )
        .arg(
            Arg::with_name("POLLARD")
                .long("--pollardrho")
                .conflicts_with_all(&["ENCRYPT", "DECRYPT", "GEN", "BRUTE"])
                .help("Breaks a public key using pollard-rho heuristic"),
        )
        .arg(
            Arg::with_name("POLLARD_RAW")
                .long("--pollardrho_raw")
                .conflicts_with_all(&["ENCRYPT", "DECRYPT", "GEN", "BRUTE", "POLLARD"])
                .help("Breaks a public key using pollard-rho heuristic with an usafe implementation"),
        )
        .arg(
            Arg::with_name("FERMAT")
                .long("--fermat")
                .conflicts_with_all(&["ENCRYPT", "DECRYPT", "GEN", "BRUTE", "POLLARD"])
                .help("Breaks a public key using Fermat's factorization"),
        )
        .arg(
            Arg::with_name("FERMAT_RAW")
                .long("--fermat_raw")
                .conflicts_with_all(&["ENCRYPT", "DECRYPT", "GEN", "BRUTE", "POLLARD", "FERMAT"])
                .help("Breaks a public key using Fermat's factorization with an unsafe implementation"),
        )
        .get_matches();

    let file_name = if matches.is_present("INPUT") {
        matches
            .values_of("INPUT")
            .unwrap()
            .collect::<Vec<_>>()
            .first()
            .unwrap()
            .to_string()
    } else {
        "".to_string()
    };

    let n_bits = if matches.is_present("KEYSIZE") {
        matches
            .values_of("KEYSIZE")
            .unwrap()
            .collect::<Vec<_>>()
            .first()
            .unwrap()
            .parse::<i64>()
            .expect("Keysize must be an integer value")
    } else {
        32_i64
    };

    let key_file_name = if matches.is_present("KEY") {
        matches
            .values_of("KEY")
            .unwrap()
            .collect::<Vec<_>>()
            .first()
            .unwrap()
            .to_string()
    } else {
        "key".to_string()
    };

    if matches.is_present("GEN") {
        rsa::gen_key_and_save_to_file(n_bits, key_file_name.clone());
    };

    let file_name_len = file_name.len();

    if matches.is_present("ENCRYPT") {
        rsa::encrypt_file(
            file_name.clone(),
            format!("{}.enc", file_name.clone()),
            rsa::get_key_from_file(key_file_name.clone()),
            n_bits,
        );
    }

    if matches.is_present("DECRYPT") {
        let mut ff = file_name.clone();
        rsa::decrypt_file(
            file_name.clone(),
            format!("{}.dec", ff.drain(0..file_name_len - 4).collect::<String>()),
            rsa::get_key_from_file(key_file_name.clone()),
            n_bits,
        );
    }

    let brute = matches.is_present("BRUTE");
    let pollard = matches.is_present("POLLARD");
    let pollard_raw = matches.is_present("POLLARD_RAW");
    let fermat = matches.is_present("FERMAT");
    let fermat_raw = matches.is_present("FERMAT_RAW");

    if brute || pollard || pollard_raw || fermat || fermat_raw {
        let (_, modulus) = rsa::get_key_from_file(key_file_name.clone());

        let t_start = Instant::now();
        let factors = if pollard {
            big_primes::prime_factorization_pollard_rho(modulus)
        } else if pollard_raw {
            big_primes::prime_factorization_pollard_rho_raw(modulus)
        } else if fermat {
            big_primes::prime_factorization_fermats(modulus)
        } else if fermat_raw {
            big_primes::prime_factorization_fermats_raw(modulus)
        } else if brute {
            big_primes::prime_factorization_brute_force(modulus)
        } else {
            unreachable!();
        };

        let t_end = Instant::now();

        let diff = t_end.duration_since(t_start);

        assert_eq!(factors.len(), 4);

        println!("{:?}.{:?}", diff.as_secs(), diff.subsec_micros());

        let p = factors[1].clone();
        let q = factors[2].clone();

        //println!("{:?} {:?}", p, q);

        let key_file_name_len = key_file_name.len();
        let mut broken_key = key_file_name.clone();

        rsa::get_prv_key_from_pq_and_dump_to_file(
            p,
            q,
            broken_key
                .drain(0..key_file_name_len - 4)
                .collect::<String>(),
        );
    }
}
