extern crate cryptopals;

use cryptopals::lib;
use cryptopals::base64;

fn main() {

    print!("Challenge - 1... ");
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    let phrase = lib::hex2bin(&input);
    assert_eq!(input, lib::bin2hex(&phrase));
    assert_eq!(output, base64::encode(&phrase));

    println!("Done!");

}
