use std::ffi::CString;

use crate::bindings;

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
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

impl Into<bindings::Color> for Color {
    fn into(self) -> bindings::Color {
        return bindings::Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        };
    }
}
pub struct Context {}

impl Context {
    pub fn clear_background(&mut self, color: Color) {
        unsafe {
            bindings::ClearBackground(bindings::Color {
                r: color.r,
                g: color.g,
                b: color.b,
                a: color.a,
            });
        }
    }

    pub fn draw_text(&mut self, text: String, x: i32, y: i32, size: i32, color: Color) {
        let c_text = CString::new(text).unwrap();
        unsafe { bindings::DrawText(c_text.into_raw(), x, y, size, color.into()) }
    }
}
