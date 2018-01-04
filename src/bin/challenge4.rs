extern crate cryptopals;

use cryptopals::lib;
use std::fs::File;
use std::io::prelude::*;

fn main() {

  println!("Challenge - 4... ");
  
  let filepath = "resources/challenge4/file.txt";

  let mut file = File::open(filepath).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents);

  for line in contents.lines() {
    // println!("{:?}", line);
    let x = lib::hex2bin(line);
    let r = lib::brute_force_single_byte_cipher_xor(&x);

    if r.0 != 0 {
      continue;
    }

    println!("{:?} - {:?}", r.0, r.1);
  }

  println!("Done!");
  
}