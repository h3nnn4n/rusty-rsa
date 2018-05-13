extern crate rug;
use self::rug::integer::Order;
use self::rug::Integer;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use big_primes;

pub fn decrypt_file(path: String, out: String, priv_key: (Integer, Integer), n_bits: i64) {
    let mut file = File::open(path).unwrap();
    let mut buffer = [0_u8; 2];

    let mut out_file = File::create(out).unwrap();

    loop {
        let bytes_read = file.read(&mut buffer).unwrap();
        if bytes_read <= 0 {
            break;
        }

        if bytes_read == 1 {
            buffer[1] = 0;
        }

        println!("Wrote {} bytes: {:?}", bytes_read, buffer);
        let i = Integer::from_digits(&buffer, Order::Lsf);

        let c = decrypt_(priv_key.clone(), i.clone());

        let digits = c.to_digits::<u8>(Order::Lsf);

        out_file.write(&digits);
    }
}

pub fn encrypt_file(path: String, out: String, pub_key: (Integer, Integer), n_bits: i64) {
    let mut file = File::open(path).unwrap();
    let mut buffer = [0_u8; 2];

    let mut out_file = File::create(out).unwrap();

    loop {
        let bytes_read = file.read(&mut buffer).unwrap();
        if bytes_read <= 0 {
            break;
        }

        if bytes_read == 1 {
            buffer[1] = 0;
        }

        println!("Read {} bytes: {:?}", bytes_read, buffer);
        let i = Integer::from_digits(&buffer, Order::Lsf);

        let c = encrypt_(pub_key.clone(), i.clone());

        let digits = c.to_digits::<u8>(Order::Lsf);

        out_file.write(&digits);
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
    let e = big_primes::get_prime_with_n_bits(8);
    let d = mod_inv(e.clone(), tot.clone());

    ((d, n.clone()), (e, n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_decrypt() {
        for _ in 0..10 {
            let (private, public) = super::get_key(64);
            for _ in 0..10 {
                let m = big_primes::get_prime_with_n_bits(60);

                let c = encrypt_(public.clone(), m.clone());
                let m2 = decrypt_(private.clone(), c);

                assert_eq!(m, m2);
            }
        }
    }

    #[test]
    fn mod_inv() {
        assert_eq!(super::mod_inv(Integer::from(7), Integer::from(40)), 23);
        assert_eq!(super::mod_inv(Integer::from(42), Integer::from(2017)), 1969);
    }
}
