extern crate cryptopals;

use cryptopals::lib;

fn main() {

  println!("Challenge - 3... ");
  
  let input = &lib::hex2bin("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
  let r = lib::brute_force_single_byte_cipher_xor(input);
  println!("  BRUTE FORCE: {:?} - {:?}", r.0, r.1);

  println!("Done!");
  
}