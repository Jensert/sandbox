use crate::brush::Brush;
use crate::pixelgrid::{ChunkGrid, ChunkPosition};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
pub struct UserInterface {
    debug_enabled: bool,
}

impl UserInterface {
    pub fn new() -> Self {
        Self {
            debug_enabled: false,
        }
    }

    pub fn draw(&self, chunk_grid: &mut ChunkGrid, brush: &mut Brush, mouse_world_position: Vec2) {
        // [TODO] move all UI logic to a seperate function / struct
        widgets::Window::new(hash!(), vec2(0.0, 0.0), vec2(300.0, 300.0))
            .label("Debug window")
            .movable(true)
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, format!("FPS: {}", get_fps()).as_str());
                ui.label(
                    None,
                    format!("# Pixels: {}", chunk_grid.get_total_pixels()).as_str(),
                );
                ui.separator();
                if ui.button(None, "Reset pixelgrid") {
                    chunk_grid.clear();
                }
                ui.label(
                    None,
                    format!("Selected pixel: {}", brush.pixel_type().to_str()).as_str(),
                );

                ui.label(
                    None,
                    format!("Selected brush type: {}", brush.brush_type().as_str()).as_str(),
                );
                ui.label(None, format!("Brush size: {}", brush.size()).as_str());
                ui.label(
                    None,
                    format!("Mouse screen position: {:?}", mouse_position()).as_str(),
                );
                ui.label(
                    None,
                    format!("Mouse world position: {:?}", mouse_world_position).as_str(),
                );
                let position = ChunkPosition::from_world_position(mouse_world_position);
                ui.label(
                    None,
                    format!(
                        "Mouse chunk position: {:?}, {:?}",
                        position.chunk_key, position.chunk_coordinate
                    )
                    .as_str(),
                );
            });
    }
}
