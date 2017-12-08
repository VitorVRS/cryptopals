use std::str;

fn main() {

  let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string();
  // let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

  println!("{:?}", hex2bin(input));
}

fn hex2bin(input: String) -> String {

  let bytes: Vec<char> = input.chars().collect();
  let mut result: Vec<u8> = Vec::new();

  for chunk in bytes.chunks(2) {
    
    // arrumar erro de conversao de letra para numero
    let first = chunk[0].to_digit(16).unwrap();
    let second = chunk[1].to_digit(16).unwrap();
    
    let number = ((first << 4) | second) as u8;
    result.push(number);
  }

  // variavel result contém os bytes das letras

  String::from_utf8(result).unwrap()
}

// todo
// converter a string em hexadecimal para bytes - feito
// converter os bytes para base 64

// I'm
// 01001001 00100111 01101101
// 010010010010011101101101
// 010010 010010 011101 101101
// 18 18 29 45
//  S  S  d  t