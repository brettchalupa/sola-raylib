//! Demo of raylib 6.0's redesigned fullscreen + high-DPI handling.
//!
//! Controls:
//!   F — toggle fullscreen
//!   B — toggle borderless windowed
//!   ESC — exit

extern crate raylib;
use raylib::prelude::*;
use structopt::StructOpt;

mod options;

fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Borderless / Fullscreen (raylib 6.0)");

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_F) {
            rl.toggle_fullscreen();
        }
        if rl.is_key_pressed(KeyboardKey::KEY_B) {
            rl.toggle_borderless_windowed();
        }

        let fullscreen = rl.is_window_fullscreen();
        let maximized = rl.is_window_maximized();
        let pos = rl.get_window_position();
        let dpi = rl.get_window_scale_dpi();
        let render_w = rl.get_render_width();
        let render_h = rl.get_render_height();
        let screen_w = rl.get_screen_width();
        let screen_h = rl.get_screen_height();

        rl.draw(&thread, |mut d| {
            d.clear_background(Color::RAYWHITE);
            d.draw_text(
                "raylib 6.0 window modes",
                12,
                12,
                22,
                Color::DARKGRAY,
            );
            d.draw_text("[F] toggle fullscreen", 12, 48, 18, Color::BLACK);
            d.draw_text("[B] toggle borderless", 12, 72, 18, Color::BLACK);
            d.draw_text("[ESC] exit", 12, 96, 18, Color::BLACK);

            let lines = [
                format!("is_window_fullscreen: {}", fullscreen),
                format!("is_window_maximized: {}", maximized),
                format!("window position: ({:.0}, {:.0})", pos.x, pos.y),
                format!("screen size (logical): {} x {}", screen_w, screen_h),
                format!("render size (pixels):  {} x {}", render_w, render_h),
                format!("dpi scale: ({:.2}, {:.2})", dpi.x, dpi.y),
            ];
            for (i, line) in lines.iter().enumerate() {
                d.draw_text(line, 12, 140 + 24 * i as i32, 18, Color::DARKBLUE);
            }
        });
    }
}
