pub mod base64;
pub mod hamming;

pub mod lib {

    use hamming;

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
            result.push_str(&format!("{:02x}", byte));
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

            let operand = bytes2.get(index).unwrap();
            let number = v ^ operand;

            result.push(number);

            index = index + 1;
        }

        result
    }
    pub fn calc_char_score(letter: char) -> f32 {
        let english_letter_frequency = " etaoinshrdlcumwfgypbvkjxqz";
        let letter_scores = [
            15.0,
            8.167,
            1.492,
            2.782,
            4.253,
            12.70,
            2.228,
            2.015,
            6.094,
            6.966,
            0.153,
            0.772,
            4.025,
            2.406,
            6.749,
            7.507,
            1.929,
            0.095,
            5.987,
            6.327,
            9.056,
            2.758,
            0.978,
            2.360,
            0.150,
            1.974,
            0.074,
        ];
        let letter_lower = letter.to_lowercase().to_string();
        let mut score: f32 = 0.0;

        let position = english_letter_frequency.find(&letter_lower);

        if position.is_some() {
            let letter_position = position.unwrap() as usize;
            score = letter_scores[letter_position] as f32;
        }

        score
    }

    pub fn brute_force_single_byte_cipher_xor(input: Vec<u8>) -> (f32, Vec<u8>, u8) {

        let mut best_score: f32 = 0.0;
        let mut best_key: u8 = 0;
        let mut bytes: Vec<u8> = vec![];

        // iterate over all 'ascii' chars
        for key in 0..255 {

            let cipher = cipher_xor(&input, &vec![key]);
            let mut score: f32 = 0.0;


            for letter in &cipher {
                score += calc_char_score(char::from(*letter));
            }
            // println!("DEBUG: KEY: {:?} - SCORE: {:?} - DATA: {:?}", key, score, String::from_utf8_lossy(&cipher));

            if score > best_score {
                best_score = score;
                best_key = key;
                bytes = cipher.clone();
            }
        }

        (best_score, bytes, best_key)
    }

    pub fn find_repeating_xor_keysize(input: &[u8]) -> Vec<(usize, f32)> {
        let mut result = vec![];

        for keysize in 2..40 {
            let chunks = input.chunks_exact(keysize);

            // for each pair of chunk, we calc the hamming distance
            let folded = chunks.fold((false, "".as_bytes(), vec![]), |acc, block| {
                let mut distances = acc.2;
                if acc.0 {
                    let distance = hamming::distance(acc.1, block) as f32 / keysize as f32;
                    distances.push(distance)
                }
                (!acc.0, block, distances)
            });

            // get the sum of all hamming distances
            let distances = folded.2.iter().fold(0.0, |acc, x| acc + x);

            // normalize it dividing by the number of pairs
            let normalized_distance = distances / folded.2.len() as f32;
            result.push((keysize, normalized_distance));
        }

        result.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        result
    }

}
