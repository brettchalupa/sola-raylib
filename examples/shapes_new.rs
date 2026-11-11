//! Demo of raylib 6.0 + 6.2's new shape functions:
//!   * `draw_line_dashed` (6.0)
//!   * `draw_ellipse_v` / `draw_ellipse_lines_v` (6.0)
//!   * `draw_triangle_gradient` / `draw_triangle_lines_ex` (6.2)
//!   * `draw_circle_lines_ex` / `draw_circle_sector_lines_ex` (6.2)
//!   * `draw_ellipse_lines_ex` / `draw_ring_lines_ex` (6.2)
//!
//! Move the mouse to animate the ellipses and dashed lines.

extern crate raylib;
use raylib::prelude::*;
use structopt::StructOpt;

mod options;

fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Shapes — raylib 6.0 + 6.2 additions");
    let (w, h) = (opt.width, opt.height);
    let center = Vector2::new(w as f32 / 2.0, h as f32 / 2.0);

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mouse = rl.get_mouse_position();
        rl.draw(&thread, |mut d| {
            d.clear_background(Color::RAYWHITE);

            d.draw_text(
                "raylib 6.2 new shapes: TriangleGradient/LinesEx, CircleLinesEx,",
                10,
                10,
                18,
                Color::DARKGRAY,
            );
            d.draw_text(
                "CircleSectorLinesEx, EllipseLinesEx, RingLinesEx",
                10,
                30,
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

            // 6.2: gradient triangle in the top-left, thick outline around it.
            let t = 40.0;
            let v1 = Vector2::new(40.0, 60.0);
            let v2 = Vector2::new(40.0 + t, 60.0);
            let v3 = Vector2::new(40.0 + t * 0.5, 60.0 + t);
            d.draw_triangle_gradient(
                v1,
                v2,
                v3,
                Color::RED,
                Color::GREEN,
                Color::BLUE,
            );
            d.draw_triangle_lines_ex(v1, v2, v3, 3.0, Color::BLACK);

            // 6.2: thick circle outline around the mouse.
            d.draw_circle_lines_ex(mouse, dx * 0.6 + 20.0, 4.0, Color::DARKGREEN);

            // 6.2: thick sector outline that sweeps with the mouse angle.
            let angle = (mouse.y - center.y).atan2(mouse.x - center.x);
            d.draw_circle_sector_lines_ex(
                center,
                90.0,
                angle - 0.6,
                angle + 0.6,
                24,
                5.0,
                Color::PURPLE,
            );

            // 6.2: thick ellipse outline in the bottom-right corner.
            d.draw_ellipse_lines_ex(
                Vector2::new(w as f32 - 80.0, h as f32 - 60.0),
                60.0,
                30.0,
                3.0,
                Color::ORANGE,
            );

            // 6.2: thick ring around the center.
            d.draw_ring_lines_ex(
                center,
                140.0,
                150.0,
                0.0,
                std::f32::consts::PI,
                48,
                6.0,
                Color::DARKBLUE,
            );
        });
    }
}
