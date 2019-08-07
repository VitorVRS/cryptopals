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
  println!("{:?} - {:?} - {:?}", key_sizes[0], key_sizes[1], key_sizes[2]);

  // TODO get blocks of data KEYSIZE size.
  let KEYSIZE = key_sizes[0].0;
  let chunks = data.chunks_exact(KEYSIZE);
  let chunk_remainder = chunks.remainder();
  let mut blocks: Vec<Vec<u8>> = Vec::new();

  // iniatilize vector, for the transposed blocks
  let mut count = 0;
  while count < KEYSIZE {
    let mut item: Vec<u8> = Vec::new();
    blocks.push(item);
    count += 1;
  }
  
  // while reading the blocks, transpose them
  for chunk in chunks {
    let mut index = 0;
    while index < KEYSIZE {
      blocks[index].push(chunk[index]);
      index += 1;
    }
  }
  // println!("REMAINING DATA: {:?}", chunk_remainder);

  // for each block, use single char key brute force
  let mut key: Vec<u8> = Vec::new();
  for block in blocks {
    let broke = lib::brute_force_single_byte_cipher_xor(block);
    // println!("KEY: {:?} - SCORE: {:?}", String::from_utf8_lossy(&vec![broke.2]), broke.0);
    key.push(broke.2)
  }

  println!("{:?}", String::from_utf8_lossy(&lib::cipher_xor(&data, &key)));
  println!("Done!");
  
}