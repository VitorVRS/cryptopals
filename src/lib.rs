#[macro_use] 
extern crate lazy_static;

pub mod base64;
pub mod HexVector;

pub mod lib {

  extern crate regex;
  extern crate ascii;

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

    unsafe {
      ascii::AsciiString::from_ascii_unchecked(result).into()
    }
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

    unsafe {
      ascii::AsciiString::from_ascii_unchecked(result).into()
    }
  }

  pub fn calc_word_score(input: &str) -> i8 {
    let word = input;

    lazy_static! {
        static ref RE1: regex::Regex = regex::Regex::new(r"[^a-z'A-Z]").unwrap();
        static ref RE2: regex::Regex = regex::Regex::new(r"[aeiouAEIOU]").unwrap();
        static ref RE3: regex::Regex = regex::Regex::new(r"^.*[^aeiouAEIOU']{3}$").unwrap();
        static ref RE4: regex::Regex = regex::Regex::new(r"[^aeiouAEIOU']{4}").unwrap();
        static ref RE5: regex::Regex = regex::Regex::new(r"[aeiouAEIOU]{3}").unwrap();
    }

    let mut score: i8 = 0;

    // se tiver caracteres especiais = -10
    if RE1.is_match(&word) { score = score - 10; }

    // SE nao tiver vogais = -10
    if !RE2.is_match(&word) { score = score - 10; }

    // se terminar com tres consoantes = -1
    if RE3.is_match(&word) { score = score - 1; }

    // SE terminar com 4 consoantes = -4
    if RE4.is_match(&word) { score = score - 4; }

    // se tiver 3 vogais juntas = -5
    if RE5.is_match(&word) { score = score - 5; }

    score
  }

  pub fn brute_force_single_byte_cipher_xor(input: &str) -> (i8, String) {

    let keys = "abcdefghijklmnopqrstuvxwyzABCDEFGHIJKLMNOPQRSTUVXWYZ".chars();

    let mut best_score: i8 = -128;
    let mut phrase: String = String::from("");

    for key in keys {
      let cipher = cipher_xor(input, &key.to_string());
      let words: Vec<&str> = cipher.split(' ').collect();

      let mut phrase_score: i8 = 0;

      for word in &words {
        phrase_score += calc_word_score(word);
      }

      if phrase_score > best_score {
        best_score = phrase_score;
        phrase = cipher.clone();
      }

    } 

    (best_score, phrase)
  }

}
