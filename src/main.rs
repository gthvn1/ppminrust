mod ppm;

use core::panic;
use std::fs::File;

fn main() {
    // First step: write hello into the file
    let filename = "sample.ppm";
    let width = 10;
    let height = 10;

    let mut file = match File::create(filename) {
        Err(e) => panic!("failed to create {}: {}", filename, e),
        Ok(file) => file,
    };

    if let Err(e) = ppm::print_header(&mut file, width, height) {
        panic!("failed to write header {}: {}", filename, e);
    }

    if let Err(e) = ppm::rasterize(&mut file, width, height) {
        panic!("failed to write header {}: {}", filename, e);
    }

    println!("{} has been written.", filename);
}
