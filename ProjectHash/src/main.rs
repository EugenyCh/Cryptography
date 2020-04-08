mod sc2000;

use sc2000::*;
use ini::Ini;
use std::process::abort;

fn main() {
    let mut h0 = String::new();
    let file = Ini::load_from_file("input.ini").unwrap();
    let mut input_name = String::new();
    for (sec, prop) in file.iter() {
        for (k, v) in prop.iter() {
            match (sec, k) {
                (Some("Cypher"), "H0") => h0 = String::from(v),
                (Some("Cypher"), "HashLength") => {
                    if v != "128" {
                        println!("Hash length must be 128!");
                        abort();
                    }
                }
                (Some("Stream"), "File") => input_name = String::from(v),
                _ => {}
            };
        }
    }
    let h0 = u128::from_str_radix(&h0, 16).unwrap();
    println!("Hashing of {} with h0 = {:#034X}", input_name, h0);
    let h = hash(&input_name, h0);
    println!("Hash is {:#034X}", h);
}
