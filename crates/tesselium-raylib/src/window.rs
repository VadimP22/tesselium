use std::ffi::CString;

use crate::{raylib, DrawLoopContext, InitializationHandler};

pub struct Window {
    draw_loop_ctx: DrawLoopContext,
    initialization_ctx: InitializationHandler,
}

impl Window {
    pub fn new(width: i32, height: i32, target_fps: i32, name: &str) -> Self {
        unsafe {
            raylib::InitWindow(width, height, CString::new(name).unwrap().into_raw());
            raylib::SetTargetFPS(target_fps);
        }

        return Window {
            draw_loop_ctx: DrawLoopContext {},
            initialization_ctx: InitializationHandler {},
        };
    }

    pub fn draw_loop(&mut self, mut loop_fn: impl FnMut(&mut DrawLoopContext)) {
        unsafe {
            while !raylib::WindowShouldClose() {
                raylib::BeginDrawing();
                loop_fn(&mut self.draw_loop_ctx);
                raylib::EndDrawing();
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
