use raylib::core::game_loop;
use raylib::prelude::*;

fn main() {
    let (rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, Raylib")
        .highdpi()
        .build();

    game_loop::run(rl, thread, 60, |rl, thread| {
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, Raylib", 12, 12, 20, Color::BLACK);
    });
}
