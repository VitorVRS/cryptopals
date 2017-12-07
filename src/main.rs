fn main() {

  let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string();
  // let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string();

  hex2bin(input);
}

fn hex2bin(input: String) {

  let bytes: Vec<char> = input.chars().collect();
  let mut result: Vec<u32> = Vec::new();

  for chunk in bytes.chunks(2) {
    
    println!("0x{:?}{:?}", chunk[0], chunk[1]);

    // arrumar erro de conversao de letra para numero
    let first: u32 = chunk[0].to_string().parse().unwrap();
    let second: u32 = chunk[1].to_string().parse().unwrap();
    
  }

  // println!("{:?}", result);
  // input
}

// todo
// converter a string em hexadecimal para bytes
// converter os bytes para base 64

// I'm
// 01001001 00100111 01101101
// 010010010010011101101101
// 010010 010010 011101 101101
// 18 18 29 45
//  S  S  d  t