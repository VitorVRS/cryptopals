pub fn encode(input: &[u8]) -> String {

  let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=".chars().collect();

  let bytes: Vec<u8> = input.into();
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
      sec3 = 0x40;
      sec4 = 0x40;
    } else if pad == 1 {
      sec4 = 0x40;      
    }

    result.push(alphabet[sec1 as usize]);
    result.push(alphabet[sec2 as usize]);
    result.push(alphabet[sec3 as usize]);
    result.push(alphabet[sec4 as usize]);    
  }

  result.into_iter().collect()
}

pub fn decode(input: &str) -> Vec<u8> {
  let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=".chars().collect();
  
  let chars = input.chars();
  let mut bytes: Vec<u8> = vec![];
  let mut result: Vec<u8> = vec![];

  for letter in chars {
    let index = alphabet.iter().position(|&r| r == letter).unwrap();
    bytes.push(index as u8);
  }

  for chunk in bytes.chunks(4) {

    let octet1: u32 = chunk[0] as u32;
    let octet2: u32 = chunk[1] as u32;
    let octet3: u32 = chunk[2] as u32;
    let octet4: u32 = chunk[3] as u32;

    // println!("DEBUG: o1: {:#b} ; o2: {:#b} ; o3: {:#b} ; o4: {:#b}", octet1, octet2, octet3, octet4);

    let mut data: u32 = ( octet1 << 18 ) | ( octet2 << 12 ) | ( (0b111111 & octet3) << 6 ) | (0b111111 & octet4);

    if octet4 == 0x40 {
      data = data >> 8;
    }

    if octet3 == 0x40 {
      // data = data >> 6;
    }

    let sec1 = data >> 16;
    let sec2 = ( data & 0b00000000_11111111_11111111 ) >> 8;
    let sec3 = data & 0b00000000_00000000_11111111;

    // println!("DEBUG: {:#b} ; {:#b}-{:#b}-{:#b}", data, sec1, sec2, sec3);

    // first level padding
    if octet4 != 0x40 {
      result.push(sec1 as u8);
    }
     
    result.push(sec2 as u8);      

    // second level padding
    if octet3 != 0x40 {
      result.push(sec3 as u8);
    }

  }

  result
}

// 0b1100100  0b1101101 0b1101100 0b110000

//                   00110000
//             01101100110000
//       01101101101100110000
// 01100100

// 01100100101101101100110000
