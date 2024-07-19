pub mod rgb; // use in tests/

use rgb::Color;
use std::{fs::File, io::Write};

pub struct Ppm {
    f: File,
    w: usize,
    h: usize,
}

impl Ppm {
    pub fn create(filename: &str, width: usize, height: usize) -> Ppm {
        let file = match File::create(filename) {
            Err(e) => panic!("failed to create {}: {}", filename, e),
            Ok(file) => file,
        };

        Ppm {
            f: file,
            w: width,
            h: height,
        }
    }

    pub fn write_header(self: &mut Ppm) -> std::io::Result<()> {
        let magic = "P3 # Magic number: use ASCII for debugging\n";
        let size = format!("{} {} # width & height\n", self.w, self.h);
        let maxcolor = "255 # maximum color\n";

        self.f.write_all(magic.as_bytes())?;
        self.f.write_all(size.as_bytes())?;
        self.f.write_all(maxcolor.as_bytes())
    }

    pub fn rasterize(self: &mut Ppm) -> std::io::Result<()> {
        let mut red = Color::RED.to_string();
        red.push(' '); // we add an extra space to separate two writes

        for _ in 0..self.w {
            for _ in 0..self.h {
                self.f.write_all(red.as_bytes())?;
            }
            self.f.write_all("\n".as_bytes())?;
        }

        Ok(())
    }
}
