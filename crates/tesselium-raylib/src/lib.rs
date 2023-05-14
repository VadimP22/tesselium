mod bindings;
pub mod context;
pub use context::*;

use context::Context;
use std::ffi::CString;

pub struct Window {
    ctx: Context,
}

impl Window {
    pub fn new(width: i32, height: i32, target_fps: i32, name: &str) -> Self {
        unsafe {
            bindings::InitWindow(width, height, CString::new(name).unwrap().into_raw());
            bindings::SetTargetFPS(target_fps);
        }

        return Window { ctx: Context {} };
    }

    pub fn run(&mut self, mut loopfn: impl FnMut(&mut Context)) {
        unsafe {
            while !bindings::WindowShouldClose() {
                bindings::BeginDrawing();
                loopfn(&mut self.ctx);
                bindings::EndDrawing();
            }
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            bindings::CloseWindow();
        }
    }
}
