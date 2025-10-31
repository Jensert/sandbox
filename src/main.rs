use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::{
    main,
    prelude::*,
    rand::RandGenerator,
    ui::{hash, root_ui, widgets},
};
mod app;
mod brush;
mod pixel;
mod pixelgrid;
mod pixeltype;
use app::App;
use pixelgrid::ChunkPosition;

pub fn window_settings() -> Conf {
    Conf {
        window_title: String::from("Sandbox"),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

const CHUNK_SIZE: (usize, usize) = (160, 90);
const RENDER_SIZE: (u32, u32) = (240, 125);

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
    let mut app = App::new((width_ratio, height_ratio), &rng);
    println!("Started app with seed: {seed}");
    // Create pixelgrid with the seed
    while app.running() {
        app.handle_input(&rng);
        app.start_drawing();
        clear_background(SKYBLUE);

        app.chunks().draw();

        widgets::Window::new(hash!(), vec2(0.0, 0.0), vec2(300.0, 300.0))
            .label("Info")
            .movable(true)
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, format!("FPS: {}", get_fps()).as_str());
                ui.label(
                    None,
                    format!("# Pixels: {}", app.chunks().get_total_pixels()).as_str(),
                );
                ui.separator();
                if ui.button(None, "Reset pixelgrid") {
                    app.chunks_mut().clear();
                }
                ui.label(
                    None,
                    format!("Selected pixel: {}", app.brush().pixel_type().to_str()).as_str(),
                );

                ui.label(
                    None,
                    format!("Selected brush type: {}", app.brush().brush_type().as_str()).as_str(),
                );
                ui.label(None, format!("Brush size: {}", app.brush().size()).as_str());
                ui.label(
                    None,
                    format!("Mouse screen position: {:?}", mouse_position()).as_str(),
                );
                ui.label(
                    None,
                    format!("Mouse world position: {:?}", app.mouse_to_world()).as_str(),
                );
                let position = ChunkPosition::from_world_position(app.mouse_to_world());
                ui.label(
                    None,
                    format!(
                        "Mouse chunk position: {:?}, {:?}",
                        position.chunk_key, position.chunk_coordinate
                    )
                    .as_str(),
                );
            });

        app.stop_drawing();

        app.chunks_mut().update(&rng);

        // Everything that is drawn after app.stop_drawing() is called is drawn at screen resolution

        app.chunks().draw_borders(app.render_ratio());

        next_frame().await;
    }
}
