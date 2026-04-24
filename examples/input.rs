extern crate raylib;
use raylib::prelude::*;
use structopt::StructOpt;

mod options;

fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Input");
    let (_w, _h) = (opt.width, opt.height);
    let _rust_orange = Color::new(222, 165, 132, 255);
    let _ray_white = Color::new(255, 255, 255, 255);

    rl.set_target_fps(60);
    let mut last_key: Option<KeyboardKey> = None;
    let mut last_key_name: Option<String> = None;
    while !rl.window_should_close() {
        if let Some(k) = rl.get_key_pressed() {
            last_key_name = rl.get_key_name(k);
            last_key = Some(k);
        }
        rl.draw(&thread, |mut d| {
            d.clear_background(Color::WHITE);
            d.draw_text("Press any key...", 12, 12, 20, Color::DARKGRAY);
            if let Some(k) = last_key {
                d.draw_text(&format!("enum: {:?}", k), 12, 48, 20, Color::BLACK);
                let name = last_key_name.as_deref().unwrap_or("(no layout name)");
                d.draw_text(
                    &format!("get_key_name(): {}", name),
                    12,
                    80,
                    20,
                    Color::DARKBLUE,
                );
            }
        });
    }
}
