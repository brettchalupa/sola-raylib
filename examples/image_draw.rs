//! Demo of raylib 6.2's new image-drawing and render-texture APIs:
//!   * `Image::draw_image` / `draw_image_rec` / `draw_image_ex`
//!   * `Image::draw_line_strip`
//!   * `Image::draw_rectangle_pro`, `draw_rectangle_lines` (thin form),
//!     `draw_rectangle_gradient_ex`
//!   * `Image::draw_circle_gradient`
//!   * `RaylibHandle::load_render_texture_ex` (render texture with an
//!     explicit pixel format)
//!   * `RaylibHandle::get_codepoint`
//!
//! The CPU-side canvas is redrawn every frame and pushed to the GPU with
//! `update_texture`. The spinning sprite shows the rotation/scale paths
//! animating. Rotation parameters are in degrees, matching raylib's C API.

extern crate raylib;
use raylib::consts::PixelFormat;
use raylib::prelude::*;
use structopt::StructOpt;

mod options;

fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Image Draw Example");
    let (w, h) = (opt.width, opt.height);

    let font = rl
        .load_font(&thread, "static/alagard.png")
        .expect("couldn't load font");

    // A small checked sprite so the `draw_image_rec` source-rect crop is visible.
    let src = Image::gen_image_checked(48, 48, 12, 12, Color::RED, Color::BLUE);

    // CPU-side canvas we draw into with the new Image APIs each frame.
    let mut canvas = Image::gen_image_color(w, h, Color::RAYWHITE);
    let mut texture = rl
        .load_texture_from_image(&thread, &canvas)
        .expect("couldn't create texture from canvas");

    // raylib 6.2: render texture with an explicit pixel format
    let mut target = rl
        .load_render_texture_ex(
            &thread,
            256,
            256,
            PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8,
        )
        .expect("couldn't create render texture");

    // get_codepoint reads from the start of the string; the label starts with
    // a multi-byte 'é' so the byte count differs from the char count.
    let label = "éa raylib 6.2";
    let (first_codepoint, first_bytes) =
        RaylibHandle::get_codepoint(label).expect("label is non-empty");

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let t = rl.get_time() as f32;

        // --- CPU-side canvas, exercising the new ImageDraw* functions ---
        canvas.clear_background(Color::RAYWHITE);

        canvas.draw_image(&src, 10, 10, Color::WHITE);
        canvas.draw_image_rec(&src, Rectangle::new(0.0, 0.0, 24.0, 24.0), Vector2::new(70.0, 10.0), Color::WHITE);
        canvas.draw_image_ex(
            &src,
            Vector2::new(150.0, 34.0),
            // rotation is in degrees
            (t * 90.0) % 360.0,
            1.0 + (t.cos() * 0.5),
            Color::WHITE,
        );

        let strip: &[Vector2] = &[
            Vector2::new(10.0, 80.0),
            Vector2::new(60.0, 70.0),
            Vector2::new(110.0, 95.0),
            Vector2::new(160.0, 75.0),
            Vector2::new(210.0, 90.0),
        ];
        canvas.draw_line_strip(strip, Color::DARKGREEN);

        canvas.draw_rectangle_gradient_ex(
            Rectangle::new(240.0, 10.0, 200.0, 100.0),
            Color::RED,
            Color::GREEN,
            Color::BLUE,
            Color::YELLOW,
        );
        canvas.draw_rectangle_pro(
            Rectangle::new(300.0, 60.0, 80.0, 40.0),
            Vector2::new(40.0, 20.0),
            // rotation is in degrees
            (t * 60.0) % 360.0,
            Color::new(200, 200, 255, 255),
        );
        canvas.draw_rectangle_lines(240, 10, 200, 100, Color::BLACK);
        canvas.draw_circle_gradient(Vector2::new(500.0, 60.0), 45.0, Color::WHITE, Color::BLACK);

        canvas.draw_text_ex(
            &font,
            "image text!",
            Vector2::new(150.0, 150.0),
            32.0,
            2.0,
            Color::DARKGRAY,
        );

        // Push the redrawn canvas to the GPU.
        let colors = canvas.get_image_data();
        let pixels: Vec<u8> = colors
            .as_ref()
            .iter()
            .flat_map(|c| [c.r, c.g, c.b, c.a])
            .collect();
        texture.update_texture(&pixels).expect("canvas size matches texture");

        // --- GPU-side render texture with an explicit format ---
        rl.draw_texture_mode(&thread, &mut target, |mut d| {
            d.clear_background(Color::BLACK);
            d.draw_circle_gradient(
                128,
                128,
                120.0,
                Color::new(255, 190, 90, 255),
                Color::new(120, 40, 200, 255),
            );
        });

        rl.draw(&thread, |mut d| {
            d.clear_background(Color::BLACK);
            d.draw_texture(&texture, 0, 0, Color::WHITE);
            d.draw_texture_rec(
                &target,
                Rectangle::new(0.0, 0.0, 256.0, 256.0),
                Vector2::new(w as f32 - 270.0, h as f32 - 280.0),
                Color::WHITE,
            );
            d.draw_text(
                &format!(
                    "get_codepoint(\"{label}\") -> U+{first_codepoint:04X}, {first_bytes} bytes"
                ),
                10,
                h - 30,
                20,
                Color::BLACK,
            );
        });
    }
}
