//! Demo of raylib 6.0's `UpdateModelAnimationEx`: blend two frames of a model
//! animation together smoothly.
//!
//! Controls:
//!   SPACE (hold) — blend toward the rest pose (anim frame 0) over 1 second
//!   SPACE (release) — blend back to normal playback
//!   P — pause/unpause playback

extern crate raylib;
use raylib::prelude::*;
use structopt::StructOpt;

mod options;

const BLEND_SECONDS: f32 = 1.0;

fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Animation Blending (raylib 6.0)");

    let mut camera = Camera3D::perspective(
        Vector3::new(8.0, 8.0, 8.0),
        Vector3::new(0.0, 2.5, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    let mut model = rl
        .load_model(&thread, "static/guy/guy.iqm")
        .expect("couldn't load guy.iqm");

    // guytex.png is the diffuse map for the guy model.
    let texture = rl
        .load_texture(&thread, "static/guy/guytex.png")
        .expect("couldn't load guytex.png");
    {
        let mats = model.materials_mut();
        mats[0].maps_mut()[MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize].texture =
            *texture.as_ref();
    }

    let anims = rl
        .load_model_animations(&thread, "static/guy/guyanim.iqm")
        .expect("couldn't load guyanim.iqm");

    // Use the first animation (guy has one); the blending demo blends between
    // the moving frame and a frozen frame-0 pose of the same animation.
    let anim = &anims[0];
    let keyframe_count = anim.keyframeCount as f32;

    let mut current_frame: f32 = 0.0;
    let frame_speed: f32 = 0.5;
    let mut paused = false;

    // Blend factor: 0.0 = playing anim, 1.0 = frozen on frame 0.
    let mut blend: f32 = 0.0;

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_P) {
            paused = !paused;
        }

        if !paused {
            current_frame += frame_speed;
            if current_frame >= keyframe_count {
                current_frame = 0.0;
            }
        }

        let dt = rl.get_frame_time();
        let step = dt / BLEND_SECONDS;
        if rl.is_key_down(KeyboardKey::KEY_SPACE) {
            blend = (blend + step).min(1.0);
        } else {
            blend = (blend - step).max(0.0);
        }

        // Blend the animation's current frame with the same animation at
        // frame 0 (the rest pose). When SPACE is held, blend lerps toward 1
        // and the model smoothly collapses to the rest pose; release to lerp
        // back to the playing animation.
        rl.update_model_animation_ex(&thread, &mut model, anim, current_frame, anim, 0.0, blend);

        rl.update_camera(&mut camera, CameraMode::CAMERA_ORBITAL);

        rl.draw(&thread, |mut d| {
            d.clear_background(Color::RAYWHITE);
            {
                let mut d3 = d.begin_mode3D(&camera);
                d3.draw_model(&model, Vector3::zero(), 1.0, Color::WHITE);
                d3.draw_grid(10, 1.0);
            }
            d.draw_text(
                "HOLD [SPACE] to blend toward rest pose",
                12,
                12,
                18,
                Color::DARKGRAY,
            );
            d.draw_text("[P] pause", 12, 36, 18, Color::DARKGRAY);
            d.draw_text(
                &format!("blend: {:.2}", blend),
                12,
                60,
                18,
                Color::DARKBLUE,
            );
            d.draw_text(
                &format!("frame: {:.1} / {}", current_frame, anim.keyframeCount),
                12,
                84,
                18,
                Color::DARKBLUE,
            );
            if paused {
                d.draw_text("PAUSED", 12, 108, 18, Color::MAROON);
            }
        });
    }
}
