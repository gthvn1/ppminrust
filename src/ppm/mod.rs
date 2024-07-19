pub mod rgb; // use in tests/

use std::{fs::File, io::Write};

// Operations that can be perform by rasterize
pub enum Operation {
    Fill(usize), // Set all pixels to color as parameter
    Id(usize),   // Set pixel where x = y to color
}

pub struct Ppm {
    filename: String,
    width: usize,
    height: usize,
    matrix: Vec<Vec<usize>>,
}

impl Ppm {
    pub fn create(filename: &str, width: usize, height: usize) -> Ppm {
        Ppm {
            filename: String::from(filename), // Get our own copy
            width,
            height,
            matrix: vec![vec![0; height]; width],
        }
    }

    pub fn write(self: &mut Ppm) -> std::io::Result<()> {
        let magic = "P3 # Magic number: use ASCII for debugging\n";
        let size = format!("{} {} # width & height\n", self.width, self.height);
        let maxcolor = "255 # maximum color\n";

        let mut file = match File::create(&self.filename) {
            Err(e) => panic!("failed to create {:?}: {}", self.filename, e),
            Ok(file) => file,
        };

        file.write_all(magic.as_bytes())?;
        file.write_all(size.as_bytes())?;
        file.write_all(maxcolor.as_bytes())?;

        for y in 0..self.height {
            for x in 0..self.width {
                let mut color = rgb::Rgb::new(self.matrix[x][y]).to_string();
                color.push(' '); // Add an extra space to separate all colors
                file.write_all(color.as_bytes())?;
            }
            file.write_all("\n".as_bytes())?;
        }

        Ok(())
    }

    pub fn rasterize(self: &mut Ppm, op: Operation) {
        for y in 0..self.height {
            for x in 0..self.width {
                match op {
                    Operation::Fill(color) => self.matrix[x][y] = color,
                    Operation::Id(color) => {
                        if x == y {
                            self.matrix[x][y] = color
                        }
                    }
                }
            }
        }
    }
}
