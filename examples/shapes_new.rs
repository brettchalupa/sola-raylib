//! Demo of raylib 6.0's new shape functions:
//!   * `draw_line_dashed`
//!   * `draw_ellipse_v` / `draw_ellipse_lines_v`
//!
//! Move the mouse to animate the ellipses and dashed lines.

extern crate raylib;
use raylib::prelude::*;
use structopt::StructOpt;

mod options;

fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Shapes — raylib 6.0 additions");
    let (w, h) = (opt.width, opt.height);
    let center = Vector2::new(w as f32 / 2.0, h as f32 / 2.0);

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mouse = rl.get_mouse_position();
        rl.draw(&thread, |mut d| {
            d.clear_background(Color::RAYWHITE);

            d.draw_text(
                "raylib 6.0 new shapes: DrawLineDashed, DrawEllipseV, DrawEllipseLinesV",
                10,
                10,
                18,
                Color::DARKGRAY,
            );

            // Dashed lines radiating from the screen center to the mouse.
            d.draw_line_dashed(center, mouse, 8, 6, Color::MAROON);
            d.draw_line_dashed(
                Vector2::new(center.x, 60.0),
                mouse,
                4,
                4,
                Color::DARKBLUE,
            );
            d.draw_line_dashed(
                Vector2::new(center.x, h as f32 - 20.0),
                mouse,
                2,
                10,
                Color::DARKGREEN,
            );

            // Ellipse at the mouse position, scaled by distance from center.
            let dx = (mouse.x - center.x).abs().max(8.0);
            let dy = (mouse.y - center.y).abs().max(8.0);
            d.draw_ellipse_v(mouse, dx * 0.5, dy * 0.5, Color::SKYBLUE);
            d.draw_ellipse_lines_v(mouse, dx * 0.5, dy * 0.5, Color::DARKBLUE);

            // Static filled/outlined ellipse for reference.
            d.draw_ellipse_v(center, 30.0, 20.0, Color::new(200, 200, 200, 120));
            d.draw_ellipse_lines_v(center, 30.0, 20.0, Color::BLACK);
        });
    }
}
