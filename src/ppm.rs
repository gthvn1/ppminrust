use std::{fs::File, io::Write};

pub fn print_header(f: &mut File, width: usize, height: usize) -> std::io::Result<()> {
    let magic = "P3 # Magic number: use ASCII for debugging\n";
    let size = format!("{} {} # width & height\n", width, height);
    let maxcolor = "255 # maximum color\n";

    f.write_all(magic.as_bytes())?;
    f.write_all(size.as_bytes())?;
    f.write_all(maxcolor.as_bytes())
}

pub fn rasterize(f: &mut File, width: usize, height: usize) -> std::io::Result<()> {
    for _ in 0..width {
        for _ in 0..height {
            f.write_all("0 255 0 ".as_bytes())?;
        }
        f.write_all("\n".as_bytes())?;
    }

    Ok(())
}
