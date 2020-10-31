extern crate cryptopals;

use cryptopals::base64;
use cryptopals::lib;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    let filepath = "resources/challenge6/file.txt";

    let mut file = File::open(filepath).unwrap();
    let mut contents = String::new();
    let result = file.read_to_string(&mut contents);

    if result.is_err() {
        return;
    }

    // decoded data
    let data = base64::decode(&contents.replace("\n", ""));

    // pontential keysizes in priority order
    let key_sizes = lib::find_repeating_xor_keysize(&data);

    let keysize = key_sizes[0].0;
    let chunks: Vec<&[u8]> = data.chunks_exact(keysize).collect();
    let chunk_count = chunks.clone().len();

    let mut blocks: Vec<Vec<u8>> = Vec::new();

    // transpose blocks
    let mut index = 0;
    while index < keysize {
        let mut item: Vec<u8> = Vec::new();

        let mut chunk_index = 0;
        while chunk_index < chunk_count {
            item.push(chunks[chunk_index][index]);
            chunk_index += 1;
        }

        blocks.push(item);
        index += 1;
    }

    // for each block, use single char key brute force
    let mut key: Vec<u8> = Vec::new();
    let mut index = 0;
    for block in blocks {
        let broke = lib::brute_force_single_byte_cipher_xor(block);
        println!(
            "DATA: {:?} - KEY: {:?} - BLOCK: {:?}",
            String::from_utf8_lossy(&broke.1),
            String::from_utf8_lossy(&vec![broke.2]),
            index
        );
        key.push(broke.2);
        index += 1;
    }

    println!("{:?}", String::from_utf8_lossy(&key));
    println!(
        "{:?}",
        String::from_utf8_lossy(&lib::cipher_xor(&data, &key))
    );
    println!("Done!");

}
