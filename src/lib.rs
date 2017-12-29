#[macro_use] 
extern crate lazy_static;

pub mod lib {

  extern crate regex;

  // trasnforms an hex encoded string into plain text string
  pub fn hex2bin(input: &str) -> String {

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
  pub fn bin2hex(input: &str) -> String {

    let bytes: &[u8] = input.as_bytes();
    let mut result: String = String::from("");

    for byte in bytes {
      result.push_str(&format!("{:x}", byte));
    }

    result
  }

  // transform an string into base64 encoded string
  pub fn base64encode(input: &str) -> String {

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

  pub fn cipher_xor(input: &str, key: &str) -> String {

    let bytes1: &[u8] = input.as_bytes();
    let bytes2: &[u8] = key.as_bytes();
    let mut result: Vec<u8> = vec![];
    let mut index: usize = 0;

    for v in bytes1 {

      if bytes2.get(index).is_none() {
        index = 0;
      }
// 
      let operand = bytes2.get(index).unwrap();
      let number = v ^ operand;
     
      result.push(number);

      index = index + 1;
    }

    String::from_utf8(result).unwrap()
  }

  pub fn calc_word_score(input: &str) -> i8 {
    let word = input.to_lowercase();

    lazy_static! {
        static ref re1: regex::Regex = regex::Regex::new(r"[^a-z']").unwrap();
        static ref re2: regex::Regex = regex::Regex::new(r"[aeiou]").unwrap();
        static ref re3: regex::Regex = regex::Regex::new(r"^.*[^aeiou']{3}$").unwrap();
        static ref re4: regex::Regex = regex::Regex::new(r"[^aeiou']{4}").unwrap();
        static ref re5: regex::Regex = regex::Regex::new(r"[aeiou]{3}").unwrap();
    }

    let mut score: i8 = 0;

    // se tiver caracteres especiais = -10
    if re1.is_match(&word) { score = score - 10; }

    // se nao tiver vogais = -10
    if !re2.is_match(&word) { score = score - 10; }

    // se terminar com tres consoantes = -1
    if re3.is_match(&word) { score = score - 1; }

    // se terminar com 4 consoantes = -4
    if re4.is_match(&word) { score = score - 4; }

    // se tiver 3 vogais juntas = -5
    if re5.is_match(&word) { score = score - 5; }

    score
  }

}
