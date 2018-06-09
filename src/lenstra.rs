extern crate rand;
use self::rand::Rng;

fn gcd(m: i64, n: i64) -> i64 {
    if m == 0 {
        n.abs()
    } else {
        gcd(n % m, m)
    }
}

fn divmod(a: i64, b: i64) -> (i64, i64) {
    let q = a / b;
    let r = a % b;

    (q, r)
}

pub fn modular_inv(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (1, 0, a);
    }

    let (q, r) = divmod(a, b);
    let (x, y, g) = modular_inv(b, r);

    (x, x - q * y, g)
}

pub fn elliptic_add(
    p: (i64, i64, i64),
    q: (i64, i64, i64),
    a: i64,
    _b: i64,
    m: i64,
) -> (i64, i64, i64) {
    if p.2 == 0 {
        return q;
    }

    if q.2 == 0 {
        return p;
    }

    let num;
    let denom;

    if p.0 == q.0 {
        if (p.1 + q.1) % m == 0 {
            return (0, 1, 0);
        }

        num = (3 * p.0 * p.0 + a) % m;
        denom = (2 * p.1) % m;
    } else {
        num = (q.1 - p.1) % m;
        denom = (q.0 - p.0) % m;
    }

    let (inv, _, g) = modular_inv(denom, m);

    if g > 1 {
        return (0, 0, denom);
    }

    let z = (num * inv * num * inv - p.0 - q.0) % m;

    (z, (num * inv * (p.0 - z) - p.1) % m, 1)
}

pub fn elliptic_mul(mut k: i64, mut p: (i64, i64, i64), a: i64, b: i64, m: i64) -> (i64, i64, i64) {
    let mut r = (0, 1, 0);

    while k > 0 {
        if p.2 > 1 {
            return p;
        }

        if k % 2 == 1 {
            r = elliptic_add(p, r, a, b, m);
        }

        k = k / 2;
        p = elliptic_add(p, p, a, b, m);
    }

    r
}

pub fn primes(limit: usize) -> Vec<i64> {
    let max = (1.0 + limit as f64 / (limit as f64).ln()) as usize;
    let mut prime_list: Vec<i64> = Vec::new();
    let mut primes = Vec::with_capacity(max as usize);
    let mut items_pushed = 0;

    loop {
        primes.push(true);
        items_pushed += 1;
        if items_pushed == max {
            break;
        }
    }

    primes[0] = false;
    if max > 1 {
        primes[1] = false;
    }

    for i in 0..max {
        if primes[i] {
            let mut mult = i << 1;
            while mult < max {
                primes[mult] = false;
                mult += i;
            }
        }
    }

    for (n, &prime) in primes.iter().enumerate() {
        if prime {
            prime_list.push(n as i64);
        }
    }

    prime_list
}

pub fn lenstra(n: i64, limit: i64) -> Option<i64> {
    let mut g = n;
    let mut rng = rand::thread_rng();

    let mut q = (0, 0, 0);
    let mut a = 0;
    let mut b = 0;

    while g == n {
        q = (rng.gen_range(0, n - 1), rng.gen_range(0, n - 1), 1);
        a = rng.gen_range(0, n - 1);

        b = (q.1.pow(2) - q.0.pow(3) - a * q.0) % n;
        g = gcd(4 * a.pow(3) + 27 * b.pow(2), n);
    }

    if g > 1 {
        return Some(g);
    }

    for &p in primes(limit as usize).iter() {
        let mut pp = p;
        while pp < limit {
            q = elliptic_mul(p, q, a, b, n);

            if q.2 > 1 {
                return Some(gcd(q.2, n));
            }

            pp *= p;
        }
    }

    None
}
