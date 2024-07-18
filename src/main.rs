use core::panic;
use std::{fs::File, io::Write};

fn print_header(f: &mut File, width: usize, height: usize) -> std::io::Result<()> {
    let magic = "P3 # Magic number: use ASCII for debugging\n";
    let size = format!("{} {} # width & height\n", width, height);
    let maxcolor = "255 # maximum color\n";

    f.write_all(magic.as_bytes())?;
    f.write_all(size.as_bytes())?;
    f.write_all(maxcolor.as_bytes())
}

fn rasterize(f: &mut File, width: usize, height: usize) -> std::io::Result<()> {
    for _ in 0..width {
        for _ in 0..height {
            f.write_all("0 255 0 ".as_bytes())?;
        }
        f.write_all("\n".as_bytes())?;
    }

    Ok(())
}

fn main() {
    // First step: write hello into the file
    let filename = "sample.ppm";
    let width = 10;
    let height = 10;

    let mut file = match File::create(filename) {
        Err(e) => panic!("failed to create {}: {}", filename, e),
        Ok(file) => file,
    };

    if let Err(e) = print_header(&mut file, width, height) {
        panic!("failed to write header {}: {}", filename, e);
    }

    if let Err(e) = rasterize(&mut file, width, height) {
        panic!("failed to write header {}: {}", filename, e);
    }

    println!("{} has been written.", filename);
}
