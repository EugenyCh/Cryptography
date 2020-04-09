extern crate rand;
extern crate num_bigint;

use num_bigint::{BigInt, RandBigInt};

fn modpow(b: &BigInt, e: &BigInt, m: &BigInt) -> BigInt {
    b.modpow(&e, &m)
}

pub fn signature(h: &BigInt) -> (BigInt, BigInt, BigInt) {
    let p = BigInt::parse_bytes(b"255211775190703847597530955573826158579", 10).unwrap();
    let q = BigInt::parse_bytes(b"252991020016994668398330411224101", 10).unwrap();
    let g = modpow(&BigInt::from(2), &((&p - 1) / &q), &p);
    let mut rng = rand::thread_rng();
    let k = rng.gen_bigint_range(&BigInt::from(2), &(&q - 1));
    let x = rng.gen_bigint_range(&BigInt::from(2), &(&q - 1));
    let y = modpow(&g, &x, &p);
    let r = modpow(&g, &k, &p);
    let rho = modpow(&r, &BigInt::from(1), &q);
    let s = modpow(&(&(&rho * h + &x) / &k), &BigInt::from(1), &q);
    (r, s, y)
}

pub fn check_signature(h: &BigInt, r: &BigInt, s: &BigInt, y: &BigInt) -> bool {
    let p = BigInt::parse_bytes(b"255211775190703847597530955573826158579", 10).unwrap();
    let q = BigInt::parse_bytes(b"252991020016994668398330411224101", 10).unwrap();
    let g = modpow(&BigInt::from(2), &((&p - 1) / &q), &p);
    let rho = modpow(r, &BigInt::from(1), &q);
    let x1 = modpow(r, s, &p);
    let x2 = modpow(&(&modpow(&modpow(&g, &rho, &p), h, &p) * y), &BigInt::from(1), &p);
    println!("x1 = {}", x1);
    println!("x2 = {}", x2);
    x1 == x2
}