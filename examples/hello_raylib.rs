use raylib::core::game_loop;
use raylib::prelude::*;

fn main() {
    let (rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, Raylib")
        .highdpi()
        .build();

    game_loop::run(rl, thread, 60, |rl, thread| {
        // Programmatic quit. ESC and the OS close button still work as the
        // raylib defaults; this is how you'd wire a quit menu item, gamepad
        // button, or any other in-game exit path on native or web.
        if rl.is_key_pressed(KeyboardKey::KEY_Q) {
            rl.request_quit();
        }

        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, Raylib", 12, 12, 20, Color::BLACK);
        d.draw_text("Press Q to quit", 12, 40, 20, Color::DARKGRAY);
    });
}
