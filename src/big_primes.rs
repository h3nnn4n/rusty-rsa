#![allow(dead_code)]

//extern crate rand;
extern crate rug;
//use self::rand::Rng;
//use self::rug::rand::RandState;
use self::rug::Integer;

//fn ninja_factor(n: Integer) -> (Integer, Integer) {
//let mut s = Integer::from(0);
//let mut d: Integer = n - 1;

//while d % 2 == 0 {
//d /= 2;
//s += 1;
//}

////(s.clone(), d.clone())
//(s, d)
//}

fn power_(a: Integer, n: Integer, m: Integer) -> Integer {
    let mut nn = n;
    let mut power = a;
    let mut result = Integer::from(1);

    loop {
        if Integer::from(&nn % 2) == 0 {
            break;
        }
        if Integer::from(&nn % 2) == 1 {
            let result = Integer::from(Integer::from(&result * &result) % &m);
        }

        let power = Integer::from(Integer::from(&power * &power) % &m);

        let nn = Integer::from(&nn >> 1);
    }

    result
}

//fn power(a: Integer, n: Integer, m: Integer) -> Integer {
//let mut nn = n;
//let mut power = a;
//let mut result = Integer::from(1);

//while nn > 0 {
//if nn % 2 == 1 {
//result = (result * power) % m;
//}

//power = (power * power) % m;
//nn >>= 1;
//}

//result
//}

//fn miller_rabin(n: Integer, s: Integer, d: Integer, a: Integer) -> bool {
//let mut x = power(a, d, n);
//let mut y = Integer::from(0);

//let mut _r = s;
//while _r > 0 {
//y = power(x, Integer::from(2), n);
//if y == 1 && x != 1 && x != n - 1 {
//return false;
//}
//x = y;
//_r -= 1;
//}

//return if y == 1 { true } else { false };
//}

//pub fn is_prime(n: Integer, k: i64) -> bool {
//if (n % 2 == 0 && n != 2) || (n < 2) {
//return false;
//}
//if n <= 3 {
//return true;
//}

//let mut rng = rand::thread_rng();
///////
////let (s, d) = ninja_factor(n);
///////
//let mut ss = Integer::from(0);
//let mut dd: Integer = n - 1;

//while dd % 2 == 0 {
//dd /= 2;
//ss += 1;
//}

//let (s, d) = (ss.clone(), dd.clone());
////(s, d)
///////

//for _ in 0..k {
//let mut rand = RandState::new();
//let a = 2 + (rand.bits(32) % (n - 4));
//if !miller_rabin(n.clone(), s.clone(), d.clone(), a) {
//return false;
//}
//}

//true
//}

//pub fn is_prime_str(s: String, k: i64) -> bool {
//let nn = s.parse::<Integer>().unwrap();
//return is_prime(nn, k);
//}

//pub fn count_primes(upper: i64) -> i64 {
//let mut total = 0;
//let k = 10;

//for n in 2..upper {
//if is_prime(Integer::from(n), k) {
//total += 1
//}
//}

//total
//}
