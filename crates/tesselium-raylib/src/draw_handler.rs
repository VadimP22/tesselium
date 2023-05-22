//! This module provides all drawing-related functionality.

use std::{ffi::CString, marker::PhantomData};

use crate::{color::Color, raylib, Window};

/// Contains all drawing-related things.
/// For `PhantomData<&'a mut Window>` explanation see `InitializationHandler` docs
///
/// ### Warning
/// You should drop this if you want to display things you draw.
pub struct DrawHandler<'a> {
    phantom: PhantomData<&'a mut Window>,
}

impl<'a> DrawHandler<'a> {
    pub(crate) fn create(_window: &'a mut Window) -> Self {
        unsafe {
            raylib::BeginDrawing();
        }

        return DrawHandler {
            phantom: PhantomData::default(),
        };
    }

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

impl Drop for DrawHandler<'_> {
    fn drop(&mut self) {
        unsafe {
            raylib::EndDrawing();
        }
    }
}
