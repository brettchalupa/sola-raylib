//! Minimal rlImGui demo. Shows that `draw_imgui` works inside a raylib
//! drawing scope. Run with `just examples-imgui`.

use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 480)
        .title("sola-raylib imgui demo")
        .imgui_theme(ImGuiTheme::Dark)
        .build();

    let mut counter = 0i32;
    let mut bg = Color::new(30, 30, 40, 255);

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(bg);
        d.draw_text(
            "raylib + imgui. Move the window to convince yourself it's alive.",
            12,
            12,
            18,
            Color::RAYWHITE,
        );
        d.draw_text(&format!("counter: {}", counter), 12, 40, 20, Color::YELLOW);

        d.draw_imgui(|ui| {
            ui.window("controls")
                .size([260.0, 140.0], ::imgui::Condition::FirstUseEver)
                .position([12.0, 80.0], ::imgui::Condition::FirstUseEver)
                .build(|| {
                    if ui.button("bump counter") {
                        counter += 1;
                    }
                    if ui.button("reset") {
                        counter = 0;
                    }
                    ui.separator();
                    let mut rgb = [
                        bg.r as f32 / 255.0,
                        bg.g as f32 / 255.0,
                        bg.b as f32 / 255.0,
                    ];
                    if ui.color_edit3("bg", &mut rgb) {
                        bg.r = (rgb[0] * 255.0) as u8;
                        bg.g = (rgb[1] * 255.0) as u8;
                        bg.b = (rgb[2] * 255.0) as u8;
                    }
                });
        });
    }
}
