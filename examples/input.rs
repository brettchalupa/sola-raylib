//! Input sampler: keyboard, mouse, gamepads. Shows every live signal so you
//! can verify raylib is seeing what you are doing.

extern crate raylib;
use raylib::consts::{GamepadButton, MouseButton};
use raylib::prelude::*;
use structopt::StructOpt;

mod options;

const GAMEPAD_BUTTONS: &[(GamepadButton, &str)] = &[
    (GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_UP, "DPad-Up"),
    (GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_DOWN, "DPad-Down"),
    (GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT, "DPad-Left"),
    (GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT, "DPad-Right"),
    (GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_UP, "Y/Tri"),
    (GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN, "A/Cross"),
    (GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_LEFT, "X/Square"),
    (GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT, "B/Circle"),
    (GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_1, "LB"),
    (GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_2, "LT"),
    (GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_1, "RB"),
    (GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_2, "RT"),
    (GamepadButton::GAMEPAD_BUTTON_MIDDLE_LEFT, "Select"),
    (GamepadButton::GAMEPAD_BUTTON_MIDDLE, "Home"),
    (GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT, "Start"),
    (GamepadButton::GAMEPAD_BUTTON_LEFT_THUMB, "L-Stick"),
    (GamepadButton::GAMEPAD_BUTTON_RIGHT_THUMB, "R-Stick"),
];

const MOUSE_BUTTONS: &[(MouseButton, &str)] = &[
    (MouseButton::MOUSE_BUTTON_LEFT, "L"),
    (MouseButton::MOUSE_BUTTON_MIDDLE, "M"),
    (MouseButton::MOUSE_BUTTON_RIGHT, "R"),
    (MouseButton::MOUSE_BUTTON_SIDE, "Side"),
    (MouseButton::MOUSE_BUTTON_EXTRA, "Extra"),
    (MouseButton::MOUSE_BUTTON_FORWARD, "Fwd"),
    (MouseButton::MOUSE_BUTTON_BACK, "Back"),
];

fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Input");

    rl.set_target_fps(60);

    let mut last_key: Option<KeyboardKey> = None;
    let mut last_key_name: Option<String> = None;

    while !rl.window_should_close() {
        if let Some(k) = rl.get_key_pressed() {
            last_key_name = rl.get_key_name(k);
            last_key = Some(k);
        }

        let mouse = rl.get_mouse_position();
        let mouse_wheel = rl.get_mouse_wheel_move();
        let mouse_down: Vec<&str> = MOUSE_BUTTONS
            .iter()
            .filter(|(b, _)| rl.is_mouse_button_down(*b))
            .map(|(_, name)| *name)
            .collect();

        let mut gamepads: Vec<(i32, Option<String>, Vec<&'static str>)> = Vec::new();
        for pad in 0..4 {
            if rl.is_gamepad_available(pad) {
                let name = rl.get_gamepad_name(pad);
                let pressed: Vec<&str> = GAMEPAD_BUTTONS
                    .iter()
                    .filter(|(b, _)| rl.is_gamepad_button_down(pad, *b))
                    .map(|(_, label)| *label)
                    .collect();
                gamepads.push((pad, name, pressed));
            }
        }

        rl.draw(&thread, |mut d| {
            d.clear_background(Color::RAYWHITE);

            let mut y = 12;
            let section = |d: &mut RaylibDrawHandle, y: &mut i32, title: &str| {
                d.draw_text(title, 12, *y, 20, Color::DARKBLUE);
                *y += 28;
            };
            let line =
                |d: &mut RaylibDrawHandle, y: &mut i32, text: &str, color: Color, size: i32| {
                    d.draw_text(text, 24, *y, size, color);
                    *y += size + 6;
                };

            section(&mut d, &mut y, "keyboard");
            if let Some(k) = last_key {
                line(
                    &mut d,
                    &mut y,
                    &format!(
                        "last pressed: {:?} (name: {})",
                        k,
                        last_key_name.as_deref().unwrap_or("?")
                    ),
                    Color::BLACK,
                    18,
                );
            } else {
                line(&mut d, &mut y, "press any key...", Color::GRAY, 18);
            }
            y += 8;

            section(&mut d, &mut y, "mouse");
            line(
                &mut d,
                &mut y,
                &format!("pos: ({:.0}, {:.0})  wheel: {:+.1}", mouse.x, mouse.y, mouse_wheel),
                Color::BLACK,
                18,
            );
            let down = if mouse_down.is_empty() {
                "(none)".to_owned()
            } else {
                mouse_down.join(" ")
            };
            line(&mut d, &mut y, &format!("down: {}", down), Color::BLACK, 18);
            y += 8;

            section(&mut d, &mut y, "gamepads");
            if gamepads.is_empty() {
                line(
                    &mut d,
                    &mut y,
                    "no gamepads connected (plug one in)",
                    Color::GRAY,
                    18,
                );
            } else {
                for (id, name, pressed) in &gamepads {
                    line(
                        &mut d,
                        &mut y,
                        &format!(
                            "#{}: {}",
                            id,
                            name.as_deref().unwrap_or("(unnamed)")
                        ),
                        Color::BLACK,
                        18,
                    );
                    let pressed_str = if pressed.is_empty() {
                        "(no buttons down)".to_owned()
                    } else {
                        pressed.join(" ")
                    };
                    line(&mut d, &mut y, &format!("  {}", pressed_str), Color::DARKGREEN, 18);
                }
            }

            // Little mouse crosshair so position feels connected to something.
            d.draw_circle(mouse.x as i32, mouse.y as i32, 4.0, Color::MAROON);
        });
    }
}
