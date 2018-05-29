extern crate rand;
extern crate rug;
use self::rand::{thread_rng, Rng};
use self::rug::integer::Order;
use self::rug::Integer;
use std::cmp;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use big_primes;

pub fn decrypt_file(path: String, out: String, priv_key: (Integer, Integer), n_bits: i64) {
    let mut file = File::open(path).unwrap();
    let mut out_file = File::create(out).unwrap();
    let n_bytes = n_bits / 8 - 0;

    println!("Bytes read - Digits - Buffer");

    loop {
        let mut buffer: Box<[u8]> = vec![0; n_bytes as usize].into_boxed_slice();
        let bytes_read = file.read(&mut buffer).unwrap();
        if bytes_read <= 0 {
            break;
        }

        let i = Integer::from_digits::<u8>(&buffer, Order::Lsf);
        let c = decrypt_(priv_key.clone(), i.clone());
        let digits = c.to_digits::<u8>(Order::Lsf);

        let bytes_wrote = out_file.write(&digits).unwrap();

        println!(
            "Read {} bytes, wrote {} bytes, {}: {:?} {:?}",
            bytes_read,
            bytes_wrote,
            c.significant_bits(),
            digits,
            buffer,
        );
    }
}

pub fn encrypt_file(path: String, out: String, pub_key: (Integer, Integer), n_bits: i64) {
    let mut file = File::open(path).unwrap();
    let mut out_file = File::create(out).unwrap();
    let n_bytes = n_bits / 8 - 0;

    let mut in_buffer: Vec<Vec<u8>> = Vec::new();
    let mut out_buffer: Vec<Vec<u8>> = Vec::new();
    let mut max_len: usize = 0;

    println!("Bytes read - Buffer - Digits");

    loop {
        let mut buffer: Box<[u8]> = vec![0; n_bytes as usize].into_boxed_slice();
        let bytes_read = file.read(&mut buffer).unwrap();
        if bytes_read <= 0 {
            break;
        }

        in_buffer.push(buffer.clone().to_vec());
        let i = Integer::from_digits::<u8>(&buffer, Order::Lsf);
        let c = encrypt_(pub_key.clone(), i.clone());
        let digits = c.to_digits::<u8>(Order::Lsf);
        max_len = cmp::max(max_len, digits.len());

        if c >= pub_key.clone().1 {
            println!("Zoera detector detected zoera");
        }

        if digits.len() < max_len {
            println!("HAH! {} {}", digits.len(), max_len);
        }

        //if c.significant_bits() <

        //let bytes_wrote = out_file.write(&digits).unwrap();

        //println!(
        //"Read {} bytes, wrote {} bytes, {}: {:?} {:?}",
        //bytes_read,
        //bytes_wrote,
        //c.significant_bits(),
        //buffer,
        //digits
        //);
        out_buffer.push(digits);
    }

    for (k, b) in out_buffer.iter().enumerate() {
        let mut bytes_written = 0;
        if b.len() < max_len {
            let diff = max_len - b.len();
            println!("Detected a treta of size {}", diff);
            for _ in 0..diff {
                let a = [0_u8; 1];
                let bb = out_file.write(&a).unwrap();
                bytes_written += bb;
            }
        }

        let bytes_wrote = out_file.write(b).unwrap();

        println!(
            "Read {} bytes, wrote {} bytes, {}: {:?} {:?}",
            0, bytes_wrote, 0, b, in_buffer[k],
        );

        bytes_written += bytes_wrote;

        assert_eq!(bytes_written, max_len);
    }
}

fn mod_inv(a: Integer, module: Integer) -> Integer {
    let mut mn = (module.clone(), a.clone());
    let mut xy = (Integer::from(0), Integer::from(1));

    while mn.1 != 0 {
        xy = (
            xy.1.clone(),
            xy.0.clone() - Integer::from(&mn.0 / &mn.1) * xy.1.clone(),
        );
        mn = (mn.1.clone(), Integer::from(&mn.0 % &mn.1));
    }

    while xy.0.clone() < 0 {
        xy.0 = Integer::from(&xy.0 + &module);
    }

    xy.0
}

pub fn encrypt_((e, n): (Integer, Integer), m: Integer) -> Integer {
    m.pow_mod(&e, &n).unwrap()
}

pub fn decrypt_((d, n): (Integer, Integer), m: Integer) -> Integer {
    m.pow_mod(&d, &n).unwrap()
}

pub fn get_key(n_bits: i64) -> ((Integer, Integer), (Integer, Integer)) {
    let p = big_primes::get_prime_with_n_bits(n_bits / 2);
    let q = big_primes::get_prime_with_n_bits(n_bits / 2);
    let n = Integer::from(&p * &q);
    let tot = Integer::from(Integer::from(&p - 1) * Integer::from(&q - 1));
    let e = big_primes::get_prime_with_n_bits(16);
    let d = mod_inv(e.clone(), tot.clone());

    assert!(e.clone() % tot.clone() != 0);
    assert!(tot.clone() % e.clone() != 0);

    ((d, n.clone()), (e, n))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_random_string(size: i64) -> String {
        let chars = b"abcdfeghijklmnopqrstuvwxyz";
        let mut v = String::new();

        for _ in 0..size {
            v.push(thread_rng().choose(chars).cloned().unwrap().into());
        }

        v
    }

    fn create_random_file(size: i64) -> String {
        let fname = get_random_string(8);
        let mut out_file = File::create(fname.clone()).unwrap();
        let s = get_random_string(size);

        out_file.write_all(s.as_bytes());

        fname
    }

    fn encrypt_decrypt_file() {
        for n_bits in [16, 32, 64, 128].iter() {
            for _ in 0..5 {
                let (private, public) = super::get_key(*n_bits);
                for _ in 0..10 {
                    let f = create_random_file(*n_bits);
                    let f_in = f.clone();
                    let f_enc = f.clone() + ".enc";
                    let f_dec = f + ".dec";

                    println!("{:?}", f_in);

                    encrypt_file(f_in.clone(), f_enc.clone(), public.clone(), *n_bits);
                    decrypt_file(f_enc, f_dec.clone(), private.clone(), *n_bits);

                    let mut d_in = String::new();
                    let mut d_out = String::new();
                    let mut f_in = File::open(f_in).expect("Unable to open file");
                    let mut f_out = File::open(f_dec).expect("Unable to open file");

                    f_in.read_to_string(&mut d_in)
                        .expect("Unable to read string");
                    f_out
                        .read_to_string(&mut d_out)
                        .expect("Unable to read string");

                    assert_eq!(d_in, d_out);
                }
            }
        }
    }

    #[test]
    fn encrypt_decrypt_int() {
        for n_bits in [16, 32, 64, 128].iter() {
            for _ in 0..10 {
                let (private, public) = super::get_key(*n_bits);
                for _ in 0..10 {
                    let m = big_primes::get_prime_with_n_bits(*n_bits - 10);

                    let c = encrypt_(public.clone(), m.clone());
                    let m2 = decrypt_(private.clone(), c.clone());

                    assert_eq!(
                        m, m2,
                        "priv: {:?}  pub: {:?}  m:{:?}  c:{:?}",
                        private, public, m, c
                    );
                }
            }
        }
    }

    #[test]
    fn idempotent_digit_convesion() {
        // Asserts that the to_digits <-> from_digits conversion is idempotent
        for i in 0..100 {
            let n = big_primes::get_number_with_n_bits((i * 12342 + 2456) % 128 + 32);
            let a = n.clone().to_digits::<u8>(Order::Lsf);
            let b = Integer::from_digits::<u8>(&a, Order::Lsf);
            assert_eq!(n, b);
        }
    }

    #[test]
    fn mod_inv() {
        assert_eq!(super::mod_inv(Integer::from(7), Integer::from(40)), 23);
        assert_eq!(super::mod_inv(Integer::from(42), Integer::from(2017)), 1969);
    }
}
