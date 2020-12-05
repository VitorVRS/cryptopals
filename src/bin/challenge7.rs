extern crate cryptopals;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Challenge - 7... ");

    let filepath = "resources/challenge7/file.txt";

    let mut file = File::open(filepath).unwrap();
    let mut contents = String::new();
    let result = file.read_to_string(&mut contents);

    if result.is_err() {
        return;
    }

    println!("Done!")
}
