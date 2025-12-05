use crate::brush::Brush;
use crate::mapgenerator::MapGenerator;
use crate::pixelgrid::{ChunkGrid, ChunkPosition};
use macroquad::prelude::*;
use macroquad::rand::RandGenerator;
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

    pub fn toggle_debug(&mut self) {
        self.debug_enabled = !self.debug_enabled;
    }

    /// Wrapper function for all UserInterface drawing logic
    /// This calls other internal drawing functions inside UserInterface like draw_debug
    pub fn draw(
        &self,
        chunk_grid: &mut ChunkGrid,
        map_generator: &MapGenerator,
        brush: &mut Brush,
        mouse_world_position: Vec2,
        rng: &RandGenerator,
    ) {
        if self.debug_enabled {
            self.draw_debug(chunk_grid, map_generator, brush, mouse_world_position, &rng)
        }
    }

    /// This function is called internally by UserInterface.draw()
    /// This should probably not be called directly
    fn draw_debug(
        &self,
        chunk_grid: &mut ChunkGrid,
        map_generator: &MapGenerator,
        brush: &mut Brush,
        mouse_world_position: Vec2,
        rng: &RandGenerator,
    ) {
        widgets::Window::new(hash!(), vec2(0.0, 0.0), vec2(300.0, 300.0))
            .label("Debug window")
            .movable(true)
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                // FPS
                ui.label(None, format!("FPS: {}", get_fps()).as_str());
                // Total pixels in world
                ui.label(
                    None,
                    format!("# Pixels: {}", chunk_grid.get_total_pixels()).as_str(),
                );
                ui.separator();

                // Empty pixel grid
                if ui.button(None, "Reset pixelgrid") {
                    chunk_grid.clear();
                }
                if ui.button(None, "Generate map") {
                    map_generator.generate_map(chunk_grid, rng);
                }
                // Current selected pixel to be drawn
                ui.label(
                    None,
                    format!("Selected pixel: {}", brush.pixel_type().to_str()).as_str(),
                );
                // Current brush type
                ui.label(
                    None,
                    format!("Selected brush type: {}", brush.brush_type().as_str()).as_str(),
                );
                // Current brush size
                ui.label(None, format!("Brush size: {}", brush.size()).as_str());
                // Mouse position (in screen pixels)
                ui.label(
                    None,
                    format!("Mouse screen position: {:?}", mouse_position()).as_str(),
                );
                // Mouse position (in world coordinates)
                ui.label(
                    None,
                    format!("Mouse world position: {:?}", mouse_world_position).as_str(),
                );
                let position = ChunkPosition::from_world_position(mouse_world_position);
                // Mouse position (in chunk coordinates)
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
