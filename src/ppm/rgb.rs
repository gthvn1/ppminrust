pub struct Color;
impl Color {
    pub const BLACK: RGB = RGB { r: 0, g: 0, b: 0 };
    pub const BLUE: RGB = RGB {
        r: 0,
        g: 94,
        b: 184,
    };
    pub const GREEN: RGB = RGB { r: 255, g: 0, b: 0 };
    pub const RED: RGB = RGB { r: 255, g: 0, b: 0 };
    pub const WHITE: RGB = RGB { r: 0, g: 0, b: 0 };
}

#[allow(clippy::upper_case_acronyms)]
pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB {
    pub fn new(hexcolor: usize) -> RGB {
        _ = hexcolor;
        RGB { r: 0, g: 0, b: 0 }
    }

    pub fn to_string(self: RGB) -> String {
        format!("{} {} {}", self.r, self.g, self.b)
    }
}
