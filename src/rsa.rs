extern crate base64;
extern crate rand;
extern crate rug;

use self::rug::integer::Order;
use self::rug::Integer;
use std::cmp;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use big_primes;

pub fn decrypt_file(path: String, out: String, priv_key: (Integer, Integer), n_bits: i64) {
    let mut file = File::open(path).unwrap();
    let mut out_file = File::create(out).unwrap();
    let n_bytes = n_bits / 8 - 0;

    loop {
        let mut buffer: Box<[u8]> = vec![0; n_bytes as usize].into_boxed_slice();
        let bytes_read = file.read(&mut buffer).unwrap();

        if bytes_read <= 0 {
            break;
        }

        let i = Integer::from_digits::<u8>(&buffer, Order::Lsf);
        let c = decrypt_(priv_key.clone(), i.clone());
        let digits = c.to_digits::<u8>(Order::Lsf);

        out_file.write(&digits).unwrap();
    }
}

#[allow(unused_variables)]
pub fn encrypt_file(path: String, out: String, pub_key: (Integer, Integer), n_bits: i64) {
    let mut file = File::open(path).unwrap();
    let mut out_file = File::create(out).unwrap();
    let mut n_bytes = n_bits / 16;
    n_bytes = if n_bytes < 1 { 1 } else { n_bytes };

    assert!(n_bytes > 0);

    let mut in_buffer: Vec<Vec<u8>> = Vec::new();
    let mut out_buffer: Vec<Vec<u8>> = Vec::new();
    let mut max_len: usize = 0;

    //let prv_key = get_last_prv_key();

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

        //let _i = Integer::from_digits::<u8>(&digits, Order::Lsf);
        //let _c = decrypt_(prv_key.clone(), _i.clone());
        //let _digits = _c.to_digits::<u8>(Order::Lsf);

        if c >= pub_key.clone().1 {
            println!("[ERR] Message is bigger than the key! Message will not be decryptable!");
            panic!();
        }

        //if i != _c || _i != c {
        //println!(
        //"Drift detected  ->  {:?} = {:?}       {:?} = {:?}   d: {:?} {:?}",
        //i, _c, _i, c, digits, _digits
        //);
        //println!("[ERR] Check failed. Message will not be decryptable!");
        //panic!();
        //}

        out_buffer.push(digits);
    }

    for b in out_buffer.iter() {
        let mut bytes_wrote;
        let mut bytes_written = 0;
        if b.len() < max_len {
            bytes_wrote = out_file.write(b).unwrap();
            let diff = max_len - b.len();
            for _ in 0..diff {
                let a = [0_u8; 1];
                let bb = out_file.write(&a).unwrap();
                bytes_written += bb;
            }
        } else {
            bytes_wrote = out_file.write(b).unwrap();
        }

        bytes_written += bytes_wrote;

        assert_eq!(bytes_written, max_len);
    }
}

fn mod_inv(a: Integer, module: Integer) -> Integer {
    let mut n = a.clone();

    match n.invert_mut(&module.clone()) {
        Ok(()) => (),
        Err(()) => unreachable!(),
    }

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

    assert_eq!(n, xy.0);

    xy.0
}

pub fn encrypt_((e, n): (Integer, Integer), m: Integer) -> Integer {
    m.pow_mod(&e, &n).unwrap()
}

pub fn decrypt_((d, n): (Integer, Integer), m: Integer) -> Integer {
    m.pow_mod(&d, &n).unwrap()
}

#[allow(dead_code)]
pub fn read_key_from_file(file_name: String) -> (Integer, Integer) {
    let data = fs::read(file_name).expect("Unable to read file");
    let decoded = &base64::decode(&data).unwrap();
    let key_data = String::from_utf8_lossy(decoded);
    let key_token = key_data.split(',').collect::<Vec<_>>();

    let a = Integer::from_str_radix(key_token[0], 10).unwrap();
    let b = Integer::from_str_radix(key_token[1], 10).unwrap();

    (a, b)
}

#[allow(dead_code)]
pub fn get_last_pub_key() -> (Integer, Integer) {
    let data = fs::read("pub_key").expect("Unable to read file");
    let decoded = &base64::decode(&data).unwrap();
    let pub_data = String::from_utf8_lossy(decoded);
    let pub_token = pub_data.split(',').collect::<Vec<_>>();

    let d = Integer::from_str_radix(pub_token[0], 10).unwrap();
    let n = Integer::from_str_radix(pub_token[1], 10).unwrap();

    (d, n)
}

#[allow(dead_code)]
pub fn get_last_prv_key() -> (Integer, Integer) {
    let data = fs::read("prv_key").expect("Unable to read file");
    let decoded = &base64::decode(&data).unwrap();
    let prv_data = String::from_utf8_lossy(decoded);
    let prv_token = prv_data.split(',').collect::<Vec<_>>();

    let d = Integer::from_str_radix(prv_token[0], 10).unwrap();
    let n = Integer::from_str_radix(prv_token[1], 10).unwrap();

    (d, n)
}

pub fn gen_key_and_save_to_file(n_bits: i64, file_name: String) {
    let (private, public) = get_key(n_bits);

    let pub_str = base64::encode(&format!("{:?},{:?}", public.0, public.1));
    let prv_str = base64::encode(&format!("{:?},{:?}", private.0, private.1));

    let mut pub_file = File::create(format!("{}.pub", file_name)).unwrap();
    let mut prv_file = File::create(format!("{}.prv", file_name)).unwrap();

    match pub_file.write(pub_str.as_bytes()) {
        Ok(_) => (),
        Err(_) => println!("[WRN] Failed to write public key to file"),
    }

    match prv_file.write(prv_str.as_bytes()) {
        Ok(_) => (),
        Err(_) => println!("[WRN] Failed to write private key to file"),
    }
}

pub fn get_key_from_file(file_name: String) -> (Integer, Integer) {
    let data = fs::read(file_name).expect("Unable to read file");
    let decoded = &base64::decode(&data).unwrap();
    let key_data = String::from_utf8_lossy(decoded);
    let key_token = key_data.split(',').collect::<Vec<_>>();

    let a = Integer::from_str_radix(key_token[0], 10).unwrap();
    let b = Integer::from_str_radix(key_token[1], 10).unwrap();

    (a, b)
}

pub fn get_prv_key_from_pq_and_dump_to_file(
    p: Integer,
    q: Integer,
    name: String,
) -> (Integer, Integer) {
    let n = Integer::from(&p * &q);
    let tot = Integer::from(Integer::from(&p - 1) * Integer::from(&q - 1));
    let e = Integer::from_str_radix("65537", 10).unwrap(); // Fixed public exponent
    let d = mod_inv(e.clone(), tot.clone());

    let private = (d, n.clone());

    let prv_str = base64::encode(&format!("{:?},{:?}", private.0, private.1));

    let mut prv_file = File::create(name.clone() + ".prv").unwrap();

    match prv_file.write(prv_str.as_bytes()) {
        Ok(_) => (),
        Err(_) => println!("[WRN] Failed to write private key to file"),
    }

    return private;
}

pub fn get_key(n_bits: i64) -> ((Integer, Integer), (Integer, Integer)) {
    // returns private, public

    loop {
        // Ensures that the key is exactly n_bits in size
        let p = big_primes::get_prime_with_n_bits(n_bits / 2);
        let q = big_primes::get_prime_with_n_bits(n_bits / 2);
        let n = Integer::from(&p * &q);

        //println!("{:?} {:?}", p.clone(), q.clone());
        //println!("{:?}", n.clone());

        let tot = Integer::from(Integer::from(&p - 1) * Integer::from(&q - 1));
        //let e = big_primes::get_prime_with_n_bits(16);
        let e = Integer::from_str_radix("65537", 10).unwrap(); // Fixed public exponent
        let d = mod_inv(e.clone(), tot.clone());

        if n.significant_bits() as i64 == n_bits {
            let (private, public) = ((d, n.clone()), (e, n));

            let pub_str = base64::encode(&format!("{:?},{:?}", public.0, public.1));
            let prv_str = base64::encode(&format!("{:?},{:?}", private.0, private.1));

            let mut pub_file = File::create("pub_key").unwrap();
            let mut prv_file = File::create("prv_key").unwrap();

            match pub_file.write(pub_str.as_bytes()) {
                Ok(_) => (),
                Err(_) => println!("[WRN] Failed to write private key to file"),
            }

            match prv_file.write(prv_str.as_bytes()) {
                Ok(_) => (),
                Err(_) => println!("[WRN] Failed to write private key to file"),
            }

            return (private, public);
        }
    }
}

#[cfg(test)]
mod tests {
    use self::rand;
    use self::rand::Rng;
    use super::*;
    use std::fs;

    #[allow(dead_code)]
    fn get_random_string(size: i64) -> String {
        let chars = b"abcdfeghijklmnopqrstuvwxyz";
        let mut v = String::new();

        for _ in 0..size {
            v.push(rand::thread_rng().choose(chars).cloned().unwrap().into());
        }

        v
    }

    #[allow(dead_code)]
    fn create_random_file(size: i64) -> String {
        let fname = format!("{}_{}", size, get_random_string(8));
        let mut out_file = File::create(fname.clone()).unwrap();
        let s = get_random_string(500);

        match out_file.write_all(s.as_bytes()) {
            Ok(_) => (),
            Err(_) => println!("Random file was not written to disk"),
        }

        fname
    }

    fn encrypt_decrypt_file(n_bits: i64) -> bool {
        let (private, public) = super::get_key(n_bits);

        let f = create_random_file(n_bits);

        let f_in = f.clone() + "";
        let f_enc = f.clone() + ".enc";
        let f_dec = f + ".dec";

        encrypt_file(f_in.clone(), f_enc.clone(), public.clone(), n_bits);
        decrypt_file(f_enc.clone(), f_dec.clone(), private.clone(), n_bits);

        let mut d_in = String::new();
        let mut d_out = String::new();

        let mut f_org = File::open(f_in.clone()).expect("Unable to open file");
        let mut f_out = File::open(f_dec.clone()).expect("Unable to open file");

        f_org
            .read_to_string(&mut d_in)
            .expect("Unable to read string");

        f_out
            .read_to_string(&mut d_out)
            .expect("Unable to read string");

        let _ = fs::remove_file(f_in);
        let _ = fs::remove_file(f_enc);
        let _ = fs::remove_file(f_dec);

        d_in == d_out
    }

    macro_rules! enc_dec_file {
    ($($name:ident: $value:expr,)*) => { $(
        #[test]
        fn $name() {
            let n = $value;
            for _ in 0..5 {
                assert!(encrypt_decrypt_file(n));
            }
        })*}
    }

    enc_dec_file! {
        encrypt_decrypt_file_32: 32,
        encrypt_decrypt_file_64: 64,
        encrypt_decrypt_file_128: 128,
        encrypt_decrypt_file_256: 256,
        encrypt_decrypt_file_512: 512,
        encrypt_decrypt_file_1024: 1024,
    }

    fn encrypt_decrypt_int(n_bits: i64) -> bool {
        let (private, public) = super::get_key(n_bits);
        let m = big_primes::get_prime_with_n_bits(16);

        let c = encrypt_(public.clone(), m.clone());
        let m2 = decrypt_(private.clone(), c.clone());

        m == m2
    }

    macro_rules! enc_dec_int {
    ($($name:ident: $value:expr,)*) => { $(
        #[test]
        fn $name() {
            let n = $value;
            for _ in 0..5 {
                assert!(encrypt_decrypt_int(n));
            }
        })*}
    }

    enc_dec_int! {
        encrypt_decrypt_int_32: 32,
        encrypt_decrypt_int_64: 64,
        encrypt_decrypt_int_128: 128,
        encrypt_decrypt_int_256: 256,
        encrypt_decrypt_int_512: 512,
        encrypt_decrypt_int_1024: 1024,
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
