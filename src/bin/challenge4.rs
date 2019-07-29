extern crate cryptopals;

use cryptopals::lib;
use std::fs::File;
use std::io::prelude::*;

const MAX_THREADS: usize = 10;

fn main() {

  println!("Challenge - 4... ");
  
  let filepath = "resources/challenge4/file.txt";

  let mut file = File::open(filepath).unwrap();
  let mut contents = String::new();
  let result = file.read_to_string(&mut contents);

  if result.is_err() {
    return;
  }

  let mut best_score: u32 = 99999999;
  let mut best_phrase = vec![];  
  let mut lines = contents.lines();

  loop {

    let line_slice = (&mut lines).take(40);
    let mut should_break: bool = true;

    for line in line_slice {
      
      should_break = false;

      let x = lib::hex2bin(line);
      let r = lib::brute_force_single_byte_cipher_xor(x);

      let score = r.0;
      let bytes = r.1;

      if score < best_score {
        best_score = score;
        best_phrase = bytes.clone();
      }

    }

    if should_break {
      break;
    }
  }

  println!("  BRUTE FORCE: {:?} - {:?}", best_score, String::from_utf8_lossy(&best_phrase));

  println!("Done!");
  
}
