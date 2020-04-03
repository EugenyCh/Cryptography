mod sc2000;

//use sc2000::*;
use ini::Ini;
use std::process::abort;

fn main() {
    let mut key = String::new();
    let file = Ini::load_from_file("input.ini").unwrap();
    for (sec, prop) in file.iter() {
        for (k, v) in prop.iter() {
            match (sec, k) {
                (Some("Cypher"), "Key") => key = String::from(v),
                (Some("Cypher"), "KeyLength") => {
                    if v != "128" {
                        println!("Key length must be 128!");
                        abort();
                    }
                },
                _ => {}
            };
        }
    }
    let mut key = u128::from_str_radix(&key, 16).unwrap();
    println!("{:#018x}", key);
}
