use std::ffi::CString;

use crate::{raylib, DrawHandler, InitializationHandler};

pub struct Window {}

impl Window {
    pub fn new(width: i32, height: i32, target_fps: i32, name: &str) -> Self {
        unsafe {
            raylib::InitWindow(width, height, CString::new(name).unwrap().into_raw());
            raylib::SetTargetFPS(target_fps);
        }

        return Window {};
    }

    pub fn get_initialization_handler(&mut self) -> InitializationHandler {
        return InitializationHandler::create(self);
    }

    pub fn get_draw_handler(&mut self) -> DrawHandler {
        return DrawHandler::create(self);
    }

    pub fn event_loop(&mut self, mut loop_fn: impl FnMut(&mut DrawHandler)) {
        unsafe {
            while !raylib::WindowShouldClose() {
                // Draw handler must be inside brackets because we want it to drop in the end of iteration.
                let mut draw_handler = DrawHandler::create(self);
                loop_fn(&mut draw_handler);
            }
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            raylib::CloseWindow();
        }
    }
}
