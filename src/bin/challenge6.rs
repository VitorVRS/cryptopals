extern crate cryptopals;

use cryptopals::base64;
// use cryptopals::hamming;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  
  println!("{:?}", base64::encode("vitor".as_bytes()));
  base64::decode("dml0b3I=");

  let filepath = "resources/challenge6/file.txt";

  let mut file = File::open(filepath).unwrap();
  let mut contents = String::new();
  let result = file.read_to_string(&mut contents);

  if result.is_err() {
    return;
  }

  // TODO: impl base64 decode, to decode file 
  
  // TODO: define KEYSIZE as a number between 2 and 40 (maybe get the key size as input)
  
  // TODO: impl Hamming distance algorithm
  // hamming::distance(a.as_bytes(), b.as_bytes())
  
  // TODO: take first and second chunk of data, each chunk will KEYSIZE size
  
  

  println!("Done!");
  
}