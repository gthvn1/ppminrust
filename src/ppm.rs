use std::{fs::File, io::Write};

#[allow(clippy::upper_case_acronyms)]
pub struct PPM {
    f: File,
    w: usize,
    h: usize,
}

impl PPM {
    pub fn create(filename: &str, width: usize, height: usize) -> PPM {
        let file = match File::create(filename) {
            Err(e) => panic!("failed to create {}: {}", filename, e),
            Ok(file) => file,
        };

        PPM {
            f: file,
            w: width,
            h: height,
        }
    }

    pub fn write_header(self: &mut PPM) -> std::io::Result<()> {
        let magic = "P3 # Magic number: use ASCII for debugging\n";
        let size = format!("{} {} # width & height\n", self.w, self.h);
        let maxcolor = "255 # maximum color\n";

        self.f.write_all(magic.as_bytes())?;
        self.f.write_all(size.as_bytes())?;
        self.f.write_all(maxcolor.as_bytes())
    }

    pub fn rasterize(self: &mut PPM) -> std::io::Result<()> {
        for _ in 0..self.w {
            for _ in 0..self.h {
                self.f.write_all("0 255 0 ".as_bytes())?;
            }
            self.f.write_all("\n".as_bytes())?;
        }

        Ok(())
    }
}
