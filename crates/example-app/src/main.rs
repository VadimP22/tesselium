use tesselium_raylib::*;

fn main() {
    let mut w = Window::new(800, 450, 60, "Hello, window!");

    //let init = w.get_initialization_handler();
    //drop(init);

    w.event_loop(|draw| {
        draw.clear_background(Color::white());
        draw.draw_text(
            "Congrats! You created your first window!".to_owned(),
            190,
            200,
            20,
            Color::black(),
        );
    });

    println!("Hello, world!");
}
