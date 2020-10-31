pub fn distance(a: &[u8], b: &[u8]) -> u8 {
    let mut result = 0;

    let size = a.len();
    let mut index = 0;

    while index < size {
        result += (a[index] ^ b[index]).count_ones();
        index += 1;
    }

    result as u8
}
