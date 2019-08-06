extern crate cryptopals;

use cryptopals::base64;
use cryptopals::lib;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  
  let filepath = "resources/challenge6/file.txt";

  let mut file = File::open(filepath).unwrap();
  let mut contents = String::new();
  let result = file.read_to_string(&mut contents);

  if result.is_err() {
    return;
  } 

  // decoded data
  let data = base64::decode(&contents.replace("\n", ""));
  
  // pontential keysizes in priority order
  let key_sizes = lib::find_repeating_xor_keysize(&data);
  println!("{:?}", key_sizes);

  // TODO get blocks of data KEYSIZE size.
  // TODO transpose block
  // for each block, use single char key brute force

  println!("Done!");
  
}