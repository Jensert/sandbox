use std::fs::read_to_string;

use crate::RENDER_SIZE;
use crate::app::AppMode;
use crate::brush::Brush;
use crate::mapgenerator::MapGenerator;
use crate::pixelgrid::{ChunkGrid, ChunkPosition};
use crate::pixeltype::PixelType;
use macroquad::prelude::*;
use macroquad::rand::RandGenerator;
use macroquad::ui::{hash, root_ui, widgets};
pub struct UserInterface {
    debug_enabled: bool,
    shader_enabled: bool,
    vertex_shader: String,
    fragment_shader: String,
    zoom_enabled: bool,
    zoom: f32,
}
pub struct UserInterfaceData {
    pub debug_enabled: bool,
    pub shader_enabled: bool,
    pub vertex_shader: String,
    pub fragment_shader: String,
    pub zoom_enabled: bool,
    pub zoom: f32,
}

impl UserInterface {
    pub fn new() -> Self {
        Self {
            debug_enabled: true,
            shader_enabled: false,
            vertex_shader: read_to_string("src/shaders/vertex.glsl")
                .expect("expected a vertex glsl shader"),
            fragment_shader: read_to_string("src/shaders/texture.frag")
                .expect("expected a fragment glsl shader"),
            zoom_enabled: false,
            zoom: 7.0,
        }
    }

    pub fn toggle_debug(&mut self) {
        self.debug_enabled = !self.debug_enabled;
    }

    pub fn toggle_shader(&mut self) {
        self.shader_enabled = !self.shader_enabled;
    }

    pub fn read_shader_files(&mut self) {
        println!("Reloading shader files");
        self.fragment_shader =
            read_to_string("src/shaders/texture.frag").expect("expected a fragment glsl shader");
        self.vertex_shader =
            read_to_string("src/shaders/vertex.glsl").expect("expected a vertex glsl shader");
    }

    pub fn data(&self) -> UserInterfaceData {
        UserInterfaceData {
            debug_enabled: self.debug_enabled,
            shader_enabled: self.shader_enabled,
            vertex_shader: self.vertex_shader.clone(),
            fragment_shader: self.fragment_shader.clone(),
            zoom_enabled: self.zoom_enabled,
            zoom: self.zoom.clone(),
        }
    }

    pub fn enable_zoom(&mut self) {
        self.zoom_enabled = true;
    }
    pub fn disable_zoom(&mut self) {
        self.zoom_enabled = false;
    }
    pub fn zoom_increase(&mut self) {
        self.zoom += 0.1;
    }
    pub fn zoom_decrease(&mut self) {
        self.zoom -= 0.1;
    }

    /// Wrapper function for all UserInterface drawing logic
    /// This calls other internal drawing functions inside UserInterface like draw_debug
    pub fn draw(
        &mut self,
        chunk_grid: &mut ChunkGrid,
        map_generator: &MapGenerator,
        brush: &mut Brush,
        mouse_world_position: Vec2,
        rng: &RandGenerator,
        app_mode: AppMode,
        render_ratio: (f32, f32),
    ) {
        self.draw_toolbar(app_mode, render_ratio, brush);
        if self.debug_enabled {
            self.draw_debug(chunk_grid, map_generator, brush, mouse_world_position, &rng)
        }
    }

    /// This function is called internally by UserInterface.draw()
    /// This should probably not be called directly
    fn draw_debug(
        &mut self,
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
                // General technical information
                ui.separator();
                // FPS
                ui.label(None, format!("FPS: {}", get_fps()).as_str());
                // Total pixels in world
                ui.label(
                    None,
                    format!("# Pixels: {}", chunk_grid.get_total_pixels()).as_str(),
                );
                // Mouse position (in screen pixels)
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
                ui.label(None, format!("Mouse zoom: {}", self.zoom).as_str());

                // ChunkGrid functions
                ui.separator();

                if ui.button(None, "Reset pixelgrid") {
                    chunk_grid.clear();
                }
                if ui.button(None, "Generate map") {
                    map_generator.generate_map(chunk_grid, rng);
                }

                // Shader functions
                ui.separator();

                if ui.button(None, "Read shader files") {
                    self.read_shader_files();
                }
                let shader_status = match self.shader_enabled {
                    true => "ON",
                    false => "OFF",
                };
                if ui.button(None, format!("Shader is {shader_status} - Toggle shader")) {
                    self.toggle_shader();
                }

                // Brush information
                ui.separator();

                ui.label(
                    None,
                    format!("Selected pixel: {}", brush.pixel_type().name_str()).as_str(),
                );
                ui.label(
                    None,
                    format!("Selected brush type: {}", brush.brush_type().as_str()).as_str(),
                );
                ui.label(None, format!("Brush size: {}", brush.size()).as_str());
            });
    }

    pub fn draw_toolbar(&mut self, app_mode: AppMode, render_ratio: (f32, f32), brush: &mut Brush) {
        match app_mode {
            AppMode::Draw => {
                let widget_rectangle_count = PixelType::count();
                let widget_size = Vec2::new((RENDER_SIZE.0 as f32 * render_ratio.0) * 0.6, 100.0);
                let widget_rectangle_size =
                    Vec2::new(widget_size.x / widget_rectangle_count as f32, widget_size.y);
                let widget_pos = Vec2::new(
                    (RENDER_SIZE.0 as f32 * render_ratio.0) / 2.0 - widget_size.x / 2.0,
                    (RENDER_SIZE.1 as f32 * render_ratio.1) - widget_size.y - (widget_size.y * 0.1),
                );
                let mut widget_pixeltype = PixelType::first();

                widgets::Window::new(hash!(), widget_pos, widget_size)
                    .movable(false)
                    .ui(&mut *root_ui(), |ui| {
                        for pixel_count in 0..widget_rectangle_count {
                            let border_color = if brush.pixel_type() == widget_pixeltype {
                                BLACK
                            } else {
                                WHITE
                            };
                            let fill_color = widget_pixeltype.color();
                            let mut canvas = ui.canvas();
                            let cursor = canvas.cursor();
                            canvas.rect(
                                Rect::new(
                                    cursor.x
                                        + ((widget_size.x / widget_rectangle_count as f32)
                                            * pixel_count as f32),
                                    cursor.y,
                                    widget_rectangle_size.x - 1.0,
                                    widget_rectangle_size.y,
                                ),
                                border_color,
                                fill_color,
                            );
                            widget_pixeltype.next();
                        }
                    });
            }
            AppMode::Select => (),
        }
    }
}
