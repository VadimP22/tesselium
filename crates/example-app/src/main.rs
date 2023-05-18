use tesselium_raylib::*;

fn main() {
    let mut w = Window::new(800, 450, 60, "Hello, window!");

    w.draw_loop(|ctx| {
        ctx.clear_background(Color::white());
        ctx.draw_text(
            "Congrats! You created your first window!".into(),
            190,
            200,
            20,
            Color::black(),
        );
    });

    println!("Hello, world!");
}
