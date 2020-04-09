mod signature;

extern crate num_bigint;
extern crate num_traits;

use signature::*;
use num_bigint::BigInt;

fn main() {
    let h = BigInt::from(1234567890);
    let (r, s, y) = signature(&h);
    let check = check_signature(&h, &r, &s, &y);
    println!("{}", check);
}
