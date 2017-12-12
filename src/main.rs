fn main() {

  let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
  let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

  let phrase = hex2bin(&input);
  assert_eq!(input, bin2hex(&phrase));
  assert_eq!(output, base64encode(&phrase));
}

fn hex2bin(input: &str) -> String {

  let bytes: Vec<char> = input.chars().collect();
  let mut result: Vec<u8> = vec![];

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

fn bin2hex(input: &str) -> String {

  let bytes: &[u8] = input.as_bytes();
  let mut result: String = String::from("");
  
  for byte in bytes {
    result.push_str(&format!("{:x}", byte));
  }

  result
}

fn base64encode(input: &str) -> String {

  let bytes: Vec<u8> = input.as_bytes().into();
  let mut result: Vec<u8> = vec![];

  for chunk in bytes.chunks(3) {

    let mut pad: u8 = 2;
    let mut octet1: u32 = chunk[0] as u32;
    let mut octet2: u32 = 0;
    let mut octet3: u32 = 0;

    if chunk.len() >= 2 {
      octet2 = chunk[1] as u32;
      pad = 1;
    }

    if chunk.len() >= 3 {
      octet3 = chunk[2] as u32;
      pad = 0;
    }

    let data: u32 = (octet1 << 16) | (octet2 << 8) | octet3;

    let sec1: u8 = ( data >> 18 ) as u8;
    let sec2: u8 = ( ( data & 0b000000_111111_111111_111111 ) >> 12 ) as u8;
    let mut sec3: u8 = ( ( data & 0b000000_000000_111111_111111 ) >> 6 ) as u8;
    let mut sec4: u8 = (   data & 0b000000_000000_000000_111111 ) as u8;

    if pad == 2 {
      sec3 = 0x3d;
      sec4 = 0x3d;
    } else if pad == 1 {
      sec4 = 0x3d;      
    }

    result.push(sec1);
    result.push(sec2);
    result.push(sec3);
    result.push(sec4);    
  }

  String::from_utf8(result).unwrap()
}

// todo
// converter a string em hexadecimal para bytes - feito
// converter os bytes para base 64 - feito
// converter da tabela base64 para ascii

// I'm
// 01001001 00100111 01101101 00100000
// 01001001001001110110110100100000
// AAAAAA AABBBB BBBBCC CCCCCC DDDDDD DD|
// 010010 010010 011101 101101 001000 00|0000
// 18         18     29     45      8      0 
//  S          S      d      t      I      A
