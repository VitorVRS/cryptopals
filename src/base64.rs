pub fn encode(input: &str) -> String {

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
