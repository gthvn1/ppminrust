pub mod rgb; // use in tests/

use std::{fs::File, io::Write, path::Path};

pub struct Ppm<'a> {
    filename: &'a Path,
    width: usize,
    height: usize,
    matrix: Vec<Vec<usize>>,
}

impl<'a> Ppm<'a> {
    pub fn create(filename: &'a str, width: usize, height: usize) -> Ppm {
        Ppm {
            filename: Path::new(filename),
            width,
            height,
            matrix: vec![vec![0; height]; width],
        }
    }

    pub fn write(self: &mut Ppm<'a>) -> std::io::Result<()> {
        let magic = "P3 # Magic number: use ASCII for debugging\n";
        let size = format!("{} {} # width & height\n", self.width, self.height);
        let maxcolor = "255 # maximum color\n";

        let mut file = match File::create(self.filename) {
            Err(e) => panic!("failed to create {:?}: {}", self.filename, e),
            Ok(file) => file,
        };

        file.write_all(magic.as_bytes())?;
        file.write_all(size.as_bytes())?;
        file.write_all(maxcolor.as_bytes())?;

        for y in 0..self.height {
            for x in 0..self.width {
                let color = rgb::Rgb::new(self.matrix[x][y]);
                file.write_all(color.to_string().as_bytes())?;
            }
            file.write_all("\n".as_bytes())?;
        }

        Ok(())
    }

    pub fn rasterize(self: &mut Ppm<'a>) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.matrix[x][y] = y
            }
        }
    }
}
