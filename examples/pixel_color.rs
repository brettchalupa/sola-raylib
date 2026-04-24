//! Demo of raylib 6.0's `get_pixel_color` and `set_pixel_color`.
//!
//! Procedurally paints an RGB hue wheel into a raw pixel buffer using
//! `set_pixel_color`, uploads it as a texture, then uses `get_pixel_color`
//! to read back the color under the mouse cursor and displays its RGBA
//! components as text.
//!
//! This exercises both sides of the raw-pixel API: writing a color into a
//! byte buffer in a given format, and reading one back out.

extern crate raylib;
use raylib::consts::PixelFormat;
use raylib::prelude::*;
use structopt::StructOpt;

mod options;

const IMG: i32 = 256;

fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Pixel Color (raylib 6.0)");

    let img = Image::gen_image_color(IMG, IMG, Color::BLACK);
    let format = PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8;
    let bytes_per_pixel = 4usize;

    unsafe {
        let buf = std::slice::from_raw_parts_mut(
            img.data() as *mut u8,
            (IMG * IMG) as usize * bytes_per_pixel,
        );
        for y in 0..IMG {
            for x in 0..IMG {
                let idx = (y as usize * IMG as usize + x as usize) * bytes_per_pixel;
                let color = hsv_pixel(x, y);
                set_pixel_color(&mut buf[idx..idx + bytes_per_pixel], color, format)
                    .expect("slice sized exactly one pixel");
            }
        }
    }

    let texture = rl
        .load_texture_from_image(&thread, &img)
        .expect("load texture from generated image");

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mouse = rl.get_mouse_position();
        let screen_w = rl.get_screen_width();
        let screen_h = rl.get_screen_height();

        let draw_x = (screen_w - IMG) / 2;
        let draw_y = (screen_h - IMG) / 2;

        let img_x = mouse.x as i32 - draw_x;
        let img_y = mouse.y as i32 - draw_y;

        let hovered = if (0..IMG).contains(&img_x) && (0..IMG).contains(&img_y) {
            let idx = (img_y as usize * IMG as usize + img_x as usize) * bytes_per_pixel;
            let buf = unsafe {
                std::slice::from_raw_parts(
                    img.data() as *const u8,
                    (IMG * IMG) as usize * bytes_per_pixel,
                )
            };
            get_pixel_color(&buf[idx..idx + bytes_per_pixel], format)
        } else {
            None
        };

        rl.draw(&thread, |mut d| {
            d.clear_background(Color::new(24, 24, 30, 255));
            d.draw_texture(&texture, draw_x, draw_y, Color::WHITE);

            d.draw_text(
                "set_pixel_color painted the image; get_pixel_color reads it back.",
                12,
                12,
                18,
                Color::RAYWHITE,
            );
            d.draw_text("Hover the image to inspect a pixel.", 12, 36, 16, Color::GRAY);

            if let Some(c) = hovered {
                let swatch_x = 12;
                let swatch_y = screen_h - 64;
                d.draw_rectangle(swatch_x, swatch_y, 48, 48, c);
                d.draw_rectangle_lines(swatch_x, swatch_y, 48, 48, Color::RAYWHITE);
                d.draw_text(
                    &format!("R {:3}  G {:3}  B {:3}  A {:3}", c.r, c.g, c.b, c.a),
                    swatch_x + 60,
                    swatch_y + 14,
                    20,
                    Color::RAYWHITE,
                );
            }
        });
    }
}

fn hsv_pixel(x: i32, y: i32) -> Color {
    let cx = IMG as f32 / 2.0;
    let cy = IMG as f32 / 2.0;
    let dx = x as f32 - cx;
    let dy = y as f32 - cy;
    let radius = cx.min(cy);
    let dist = (dx * dx + dy * dy).sqrt() / radius;
    let angle_deg = dy.atan2(dx).to_degrees();
    let hue = (angle_deg + 360.0) % 360.0;
    let sat = dist.clamp(0.0, 1.0);
    let val = 1.0 - (dist - 1.0).max(0.0).min(1.0);
    hsv_to_rgb(hue, sat, val)
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
    let c = v * s;
    let hp = h / 60.0;
    let x = c * (1.0 - (hp % 2.0 - 1.0).abs());
    let (r1, g1, b1) = match hp as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = v - c;
    Color::new(
        ((r1 + m) * 255.0) as u8,
        ((g1 + m) * 255.0) as u8,
        ((b1 + m) * 255.0) as u8,
        255,
    )
}
