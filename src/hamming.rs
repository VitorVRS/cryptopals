pub fn distance(a: &[u8], b: &[u8]) -> f64 {
  let mut result = 0;

  // size is in bits length
  let size = a.len();
  let mut index = 0;

  while index < size { 
    result += (a[index] ^ b[index]).count_ones();
    index += 1;
  }

  result as f64 / (size * 8) as f64
}
