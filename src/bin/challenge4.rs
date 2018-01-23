extern crate cryptopals;

use cryptopals::lib;
use std::fs::File;
use std::io::prelude::*;

fn main() {

  println!("Challenge - 4... ");
  
  let filepath = "resources/challenge4/file.txt";

  let mut file = File::open(filepath).unwrap();
  let mut contents = String::new();
  let result = file.read_to_string(&mut contents);

  if result.is_err() {
    return;
  }

  for line in contents.lines() {
    // println!("{:?}", line);
    let x = lib::hex2bin(line);
    let r = lib::brute_force_single_byte_cipher_xor(x);

    let score = r.0;
    let bytes = r.1;

    break;
    if r.0 != 0 {
      // println!("{:?} - {:?}", score, String::from_utf8_lossy(&bytes));
      continue;
    }

  }

  println!("Done!");
  
}