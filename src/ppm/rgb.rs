pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

// Define create trait to allow different type when creating RGB.
// - create can be done using:
//     - usize (hexadecimal value)
//     - tuple of u8 (r,g,b)
pub trait RgbCreate {
    fn create(self) -> Rgb;
}

impl RgbCreate for usize {
    fn create(self) -> Rgb {
        Rgb {
            r: (self >> 16 & 0xFF) as u8,
            g: (self >> 8 & 0xFF) as u8,
            b: (self & 0xFF) as u8,
        }
    }
}

impl RgbCreate for (u8, u8, u8) {
    fn create(self) -> Rgb {
        Rgb {
            r: self.0,
            g: self.1,
            b: self.2,
        }
    }
}

impl Rgb {
    pub fn new<T>(color: T) -> Rgb
    where
        T: RgbCreate,
    {
        color.create()
    }

    pub fn to_string(self: Rgb) -> String {
        format!("{} {} {}", self.r, self.g, self.b)
    }
}
