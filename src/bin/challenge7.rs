extern crate cryptopals;

use cryptopals::aes;
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

    let input = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff];
    let key = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf];
    let _decrypted = aes::cipher(&input , &key);
//   let _decrypted = aes::decrypt(result, "YELLOW SUBMARINE");

    println!("Done!");
}
