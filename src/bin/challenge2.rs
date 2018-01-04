extern crate cryptopals;

use cryptopals::lib;

fn main() {

  print!("Challenge - 2... ");
  
  let input = &lib::hex2bin("1c0111001f010100061a024b53535009181c");
  let key = &lib::hex2bin("686974207468652062756c6c277320657965");

  let hex_expected_result = "746865206b696420646f6e277420706c6179";

  let encrypted = lib::cipher_xor(input, key);
  
  assert_eq!(lib::hex2bin(hex_expected_result), encrypted);
  assert_eq!(hex_expected_result, lib::bin2hex(&encrypted));

  println!("Done!");
  
}