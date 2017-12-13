fn main() {

  let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
  let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

  let phrase = hex2bin(&input);
  assert_eq!(input, bin2hex(&phrase));
  assert_eq!(output, base64encode(&phrase));
  
}

// trasnforms an hex encoded string into plain text string
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

// transforms an string into hex encoded string
fn bin2hex(input: &str) -> String {

  let bytes: &[u8] = input.as_bytes();
  let mut result: String = String::from("");
  
  for byte in bytes {
    result.push_str(&format!("{:x}", byte));
  }

  result
}

// transform an string into base64 encoded string
fn base64encode(input: &str) -> String {

  let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=".chars().collect();

  let bytes: Vec<u8> = input.as_bytes().into();
  let mut result: Vec<char> = vec![];

  for chunk in bytes.chunks(3) {

    let mut pad: u8 = 2;
    let octet1: u32 = chunk[0] as u32;
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

    let sec1 = data >> 18;
    let sec2 = ( data & 0b000000_111111_111111_111111 ) >> 12;
    let mut sec3 = ( data & 0b000000_000000_111111_111111 ) >> 6;
    let mut sec4 = data & 0b000000_000000_000000_111111;

    if pad == 2 {
      sec3 = 0x3d;
      sec4 = 0x3d;
    } else if pad == 1 {
      sec4 = 0x3d;      
    }

    result.push(alphabet[sec1 as usize]);
    result.push(alphabet[sec2 as usize]);
    result.push(alphabet[sec3 as usize]);
    result.push(alphabet[sec4 as usize]);    
  }

  result.into_iter().collect()
}