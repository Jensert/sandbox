use macroquad::{
    main,
    prelude::*,
    ui::{self, hash, root_ui, widgets},
};
mod app;
mod pixel;
mod pixel_grid;
use app::{App, window_settings};

#[main(window_settings)]
async fn main() {
    let mut app = App::new((80, 45));
    while app.running() {
        app.handle_input();
        app.start_drawing();
        clear_background(SKYBLUE);

        app.pixels().draw();

        widgets::Window::new(hash!(), vec2(0.0, 0.0), vec2(200.0, 200.0))
            .label("Info")
            .movable(true)
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, format!("FPS: {}", get_fps()).as_str());
                ui.label(
                    None,
                    format!("# Pixels: {}", app.pixels().grid().len()).as_str(),
                );
                ui.separator();
                if ui.button(None, "Reset pixelgrid") {
                    app.pixels_mut().reset();
                }
                ui.label(
                    None,
                    format!("Selected pixel: {}", app.selected_pixel().get()).as_str(),
                );
            });

        app.stop_drawing();

        app.pixels_mut().update();

        next_frame().await;
    }
}
