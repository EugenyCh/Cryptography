mod counter;

use counter::*;
use ini::Ini;
use std::process::abort;

fn main() {
    let mut key = String::new();
    let file = Ini::load_from_file("input.ini").unwrap();
    let mut input_name = String::new();
    let mut mode = String::new();
    for (sec, prop) in file.iter() {
        for (k, v) in prop.iter() {
            match (sec, k) {
                (Some("Cypher"), "Key") => key = String::from(v),
                (Some("Cypher"), "KeyLength") => {
                    if v != "128" {
                        println!("Key length must be 128!");
                        abort();
                    }
                }
                (Some("Stream"), "File") => input_name = String::from(v),
                (Some("Stream"), "Mode") => {
                    mode = String::from(v);
                    if mode.as_str() != "crypt" && mode.as_str() != "decrypt" {
                        println!("Mode must be \"crypt\" or \"decrypt\"!");
                        abort();
                    }
                }
                _ => {}
            };
        }
    }
    let key = u128::from_str_radix(&key, 16).unwrap();
    match mode.as_str() {
        "crypt" => {
            println!("Crypting of {}", input_name);
            crypt(&input_name, key);
            println!("Done!");
        },
        "decrypt" => {
            println!("Decrypting of {}", input_name);
            decrypt(&input_name, key);
            println!("Done!");
        },
        _ => {}
    }
}
