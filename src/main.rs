extern crate des;

use std::fs::File;
use std::io::{Read, Write};

use des::encrypt;

fn main() {
    let input_filename = "file.iso";
    let output_filename = input_filename.to_owned() + ".des";
    let key = b"rustrust";

    let mut input_file = File::open(input_filename).unwrap();
    let mut output_file = File::create(output_filename).unwrap();
    let mut buffer = [0; 100000];

    loop {
        let len = input_file.read(&mut buffer).unwrap();
        if len > 0 {
            let mut cipher = encrypt(&buffer, key);
            cipher.resize(len, 0);
            output_file.write(&cipher).unwrap();
        }
        else {
            break;
        }
    }
}
