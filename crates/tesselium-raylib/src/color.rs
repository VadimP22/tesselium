use crate::raylib;

pub struct Color {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
    pub(crate) a: u8,
}

impl Color {
    pub const fn white() -> Self {
        return Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        };
    }

    pub const fn black() -> Self {
        return Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        };
    }

    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        return Color {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        };
    }
}

impl Into<raylib::Color> for Color {
    fn into(self) -> raylib::Color {
        return raylib::Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        };
    }
}

impl Into<Color> for raylib::Color {
    fn into(self) -> Color {
        return Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        };
    }
}
