use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::{main, prelude::*, rand::RandGenerator};
mod app;
mod brush;
mod mapgenerator;
mod pixel;
mod pixelgrid;
mod pixeltype;
mod ui;
use app::{App, AppTimer};

pub fn window_settings() -> Conf {
    Conf {
        window_title: String::from("Sandbox"),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

const CHUNK_SIZE: (usize, usize) = (160, 90);
const RENDER_SIZE: (u32, u32) = (320, 180);
const FIXED_TIMESTEP: f32 = 1.0 / 60.0; // Target 60fps

#[main(window_settings)]
async fn main() {
    let conf = window_settings();
    let initial_width = conf.window_width;
    let initial_height = conf.window_height;
    let width_ratio = initial_width as f32 / RENDER_SIZE.0 as f32;
    let height_ratio = initial_height as f32 / RENDER_SIZE.1 as f32;
    // Create a seed and RNG
    let rng = RandGenerator::new();
    let mut seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos()
        .try_into()
        .expect("Time went too fast");
    seed = seed % 12345678;
    rng.srand(seed);
    let mut app = App::new((width_ratio, height_ratio), &rng).await;
    let mut app_timer = AppTimer::new();
    println!("Started app with seed: {seed}");
    // Create pixelgrid with the seed
    while app.running() {
        let _shader_strength = app.user_interface().data().shader_enabled;
        // Get user input
        app.handle_input();

        // Update all states and logic
        app_timer.tick(|| app.update(&rng));

        // Start draw call. Everything drawn here is drawn to the render target
        app.start_drawing();
        // All drawing logic goes after this
        clear_background(SKYBLUE);
        // Draw the pixel chunks
        app.chunks().draw();
        // Draw the UI.
        app.draw_ui(&rng);
        // Stop the current draw call
        app.stop_drawing();

        // Everything that is drawn after app.stop_drawing() is called is drawn at screen resolution
        // and uses the default camera. This is not drawn to the render target
        if app.user_interface().data().debug_enabled {
            app.chunks().draw_borders(app.render_ratio());
            app.chunks().draw_stability(app.render_ratio());
        }

        next_frame().await;
    }
}
