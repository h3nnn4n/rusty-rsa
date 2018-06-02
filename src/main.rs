extern crate clap;

use clap::{App, Arg};
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
                .required_unless("GEN")
                .index(1),
        )
        .arg(
            Arg::with_name("ENCRYPT")
                .short("-e")
                .long("--encrypt")
                .conflicts_with("Decrypt")
                .help("Encrypts a file using the specified key"),
        )
        .arg(
            Arg::with_name("DECRYPT")
                .short("-d")
                .long("--decrypt")
                .conflicts_with("Encrypt")
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
                .conflicts_with_all(&["Encrypt", "Decrypt"])
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
}
