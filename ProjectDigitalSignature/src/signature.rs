extern crate rand;
extern crate num_bigint;

use num_bigint::{BigInt, RandBigInt};

fn modpow(b: &BigInt, e: &BigInt, m: &BigInt) -> BigInt {
    b.modpow(&e, &m)
}

pub fn signature(h: &BigInt) -> bool {
    let p = BigInt::parse_bytes(b"255211775190703847597530955573826158579", 10).unwrap();
    let q = BigInt::parse_bytes(b"252991020016994668398330411224101", 10).unwrap();
    let mut rng = rand::thread_rng();
    let gamma = rng.gen_bigint_range(&BigInt::from(2), &(&p - 1));
    let g = modpow(&gamma, &((&p - 1) / &q), &p);
    let k = rng.gen_bigint_range(&BigInt::from(1), &q);
    let r = modpow(&g, &k, &p);
    let rho = &r % &q;
    let mut x = BigInt::from(2);
    while (&k - &rho * h) % &x != BigInt::from(0) {
        x += 1;
    }
    let y = modpow(&g, &x, &p);
    let mut s: BigInt;
    if &k > &(&rho * h) {
        s = &(&(&k - &rho * h) / &x) % &q;
    } else {
        s = &(&(&(&k - &rho * h) / &x) + &q * &(&(&rho * h - &k) / &x)) % &q;
    }
    println!("gamma\n = {}", gamma);
    println!("g\n = {}", g);
    println!("k\n = {}", k);
    println!("r\n = {}", r);
    println!("rho\n = {}", rho);
    println!("x\n = {}", x);
    println!("y\n = {}", y);
    println!("s\n = {}", s);

    // Checking
    let x1 = &r % &p;
    let x2 = &(
        modpow(&g, &(&rho * h), &p) * modpow(&y, &s, &p)
    ) % &p;
    println!("x1 = {}", x1);
    println!("x2 = {}", x2);
    x1 == x2
}