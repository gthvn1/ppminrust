use core::panic;
use std::{fs::File, io::Write};

fn main() {
    // First step: write hello into the file
    let filename = "sample.ppm";
    let mut file = match File::create(filename) {
        Err(e) => panic!("failed to open {}: {}", filename, e),
        Ok(file) => file,
    };

    if let Err(e) = file.write_all("Hello, Sailor!".as_bytes()) {
        panic!("failed to write into {}: {}", filename, e);
    }

    println!("{} has been written.", filename);
}
