use std::ffi::CString;

use crate::{color::Color, raylib};

pub struct DrawLoopContext {}

impl DrawLoopContext {
    pub fn clear_background(&mut self, color: Color) {
        unsafe {
            raylib::ClearBackground(raylib::Color {
                r: color.r,
                g: color.g,
                b: color.b,
                a: color.a,
            });
        }
    }

    pub fn draw_text(&mut self, text: String, x: i32, y: i32, size: i32, color: Color) {
        let c_text = CString::new(text).unwrap();
        unsafe { raylib::DrawText(c_text.into_raw(), x, y, size, color.into()) }
    }
}
