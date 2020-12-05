extern crate cryptopals;

use cryptopals::base64;
use cryptopals::crypto;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Challenge - 7... ");

    let filepath = "resources/challenge7/file.txt";

    let mut file = File::open(filepath).unwrap();
    let mut contents = String::new();
    let result = file.read_to_string(&mut contents);

    if result.is_err() {
        return;
    }

    let input = base64::decode(&contents.replace("\n", ""));
    let key = "YELLOW SUBMARINE".as_bytes();
    let decipher = crypto::ecb_aes128_decrypt(&input, key);
    println!("Decrypted:\n{}", String::from_utf8_lossy(&decipher));

    println!("Done!")
}
