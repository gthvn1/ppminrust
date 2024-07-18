pub const RED: RGB = RGB { r: 255, g: 0, b: 0 };

#[allow(clippy::upper_case_acronyms)]
pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB {
    pub fn to_string(self: RGB) -> String {
        format!("{} {} {}", self.r, self.g, self.b)
    }
}
