pub fn distance(a: &[u8], b: &[u8]) -> f64 {
  // TODO check a and b sizes

  let mut result = 0;

  let size = a.len();
  let mut index = 0;

  while index < size {
    if (a[index] != b[index]) {
      println!("Byte differ");
      result += 1;
    }
    index += 1;
  }

  result as f64 / size as f64
}