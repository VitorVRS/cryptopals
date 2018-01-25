pub mod base64;

pub mod lib {

  extern crate regex;
  extern crate ascii;

  // trasnforms an hex encoded string into plain text string
  pub fn hex2bin(input: &str) -> Vec<u8> {

    let bytes: Vec<char> = input.chars().collect();
    let mut result: Vec<u8> = vec![];

    for chunk in bytes.chunks(2) {
      
      // arrumar erro de conversao de letra para numero
      let first = chunk[0].to_digit(16).unwrap();
      let second = chunk[1].to_digit(16).unwrap();
      
      let number = ((first << 4) | second) as u8;

      result.push(number);
    }

    result
  }

  // transforms an string into hex encoded string
  pub fn bin2hex(input: &[u8]) -> String {

    let mut result: String = String::from("");

    for byte in input {
      result.push_str(&format!("{:x}", byte));
    }

    result
  }

  pub fn cipher_xor(input: &[u8], key: &[u8]) -> Vec<u8> {

    let bytes1 = input;
    let bytes2 = key;
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

    result
  }

  pub fn calc_char_score(letter: char) -> u32 {

    let english_letter_frequency = "etaonrishd .,\nlfcmugypwbvkjxqz-_!?'\"/1234567890*";
    let mut uppercase_punishment = 0;
    let letter_lower = letter.to_lowercase().to_string();
    let mut score: u32 = 255;

    if letter.is_uppercase() {
      uppercase_punishment = 3;
    }

    let position = english_letter_frequency.find(&letter_lower);

    if position.is_some() {
      score = uppercase_punishment + (position.unwrap() as u32 * 2);      
    }

    score
  }

  pub fn brute_force_single_byte_cipher_xor(input: Vec<u8>) -> (u32, Vec<u8>) {

    let mut best_score: u32 = 9999999;
    let mut bytes: Vec<u8> = vec![];

    // iterate over all 'ascii' chars
    for key in 0..255 {

      let cipher = cipher_xor(&input, &vec![key]);      
      let mut score: u32 = 0;

      for letter in &cipher {
        score += calc_char_score(char::from(*letter));
      }

      if score < best_score {
        best_score = score;
        bytes = cipher.clone();
      }
    } 


    (best_score, bytes)
  }

}
