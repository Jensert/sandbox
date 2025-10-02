use macroquad::{
    main,
    prelude::*,
    ui::{hash, root_ui, widgets},
};
mod app;
mod brush;
mod pixel;
mod pixel_grid;
use app::App;

pub fn window_settings() -> Conf {
    Conf {
        window_title: String::from("Sandbox"),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[main(window_settings)]
async fn main() {
    let render_width = 16 * 40;
    let render_height = 9 * 40;
    let chunk_width = 1920 / 4;
    let chunk_height = 1080 / 4;
    let conf = window_settings();
    let initial_width = conf.window_width;
    let initial_height = conf.window_height;
    let width_ratio = initial_width as f32 / render_width as f32;
    let height_ratio = initial_height as f32 / render_height as f32;
    let mut app = App::new(
        (render_width as u32, render_height as u32),
        (width_ratio, height_ratio),
        (chunk_width, chunk_height),
    );
    while app.running() {
        app.handle_input();
        app.start_drawing();
        clear_background(SKYBLUE);

        app.chunk_grid().draw();

        widgets::Window::new(hash!(), vec2(0.0, 0.0), vec2(300.0, 300.0))
            .label("Info")
            .movable(true)
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, format!("FPS: {}", get_fps()).as_str());
                ui.label(
                    None,
                    format!("# Pixels: {}", app.chunk_grid().get_total_pixels()).as_str(),
                );
                ui.separator();
                if ui.button(None, "Reset pixelgrid") {
                    app.chunk_grid_mut().clear();
                }
                ui.label(
                    None,
                    format!("Selected pixel: {}", app.brush().pixel_type().get()).as_str(),
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
                ui.label(
                    None,
                    format!(
                        "Mouse chunk position: {:?}",
                        app.world_to_chunk(app.mouse_to_world())
                    )
                    .as_str(),
                );
            });

        app.stop_drawing();

        app.chunk_grid_mut().update();

        next_frame().await;
    }
}
