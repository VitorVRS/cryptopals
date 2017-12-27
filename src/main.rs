extern crate cryptopals;

use cryptopals::lib;

fn main() {

  challenge1();
  // challenge2();
  challenge3();
  
}

fn challenge1() {
  let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
  let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

  let phrase = lib::hex2bin(&input);
  assert_eq!(input, lib::bin2hex(&phrase));
  assert_eq!(output, lib::base64encode(&phrase));
}

fn challenge2() {

  let input = &lib::hex2bin("1c0111001f010100061a024b53535009181c");
  let key = &lib::hex2bin("686974207468652062756c6c277320657965");

  let hex_expected_result = "746865206b696420646f6e277420706c6179";

  let encrypted = lib::cipher_xor(input, key);
  
  assert_eq!(lib::hex2bin(hex_expected_result), encrypted);
  assert_eq!(hex_expected_result, lib::bin2hex(&encrypted));
}

fn challenge3() {

  let input = &lib::hex2bin("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
  let keys = "abcdefghijklmnopqrstuvxwyzABCDEFGHIJKLMNOPQRSTUVXWYZ".chars();

  for key in keys {
    println!("{:?} - {:?}", key, lib::cipher_xor(input, &key.to_string()));
  } 

  // resposta esta no char x ou no X, descobrir como saber o certo via codigo

}