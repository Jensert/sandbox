use std::fs::read_to_string;

use crate::RENDER_SIZE;
use crate::brush::Brush;
use crate::pixel::Pixel;
use crate::pixelgrid::{ChunkGrid, ChunkPosition};
use crate::pixeltype::PixelType;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
pub struct UserInterface {
    debug_overlay_enabled: bool,
    shader_enabled: bool,
    vertex_shader: String,
    fragment_shader: String,
    zoom_enabled: bool,
    zoom: f32,
    ui_mode: UiMode,

    // actual gameplay parameters
    target_pixel: Option<Pixel>,
    camera_position: Vec2,
}

impl UserInterface {
    pub fn new() -> Self {
        Self {
            debug_overlay_enabled: false,
            shader_enabled: false,
            vertex_shader: read_to_string("src/shaders/vertex.glsl")
                .expect("expected a vertex glsl shader"),
            fragment_shader: read_to_string("src/shaders/texture.frag")
                .expect("expected a fragment glsl shader"),
            zoom_enabled: false,
            zoom: 7.0,
            ui_mode: UiMode::Select,

            target_pixel: None,
            camera_position: Vec2::new(RENDER_SIZE.0 as f32 / 2.0, RENDER_SIZE.1 as f32 / 2.0),
        }
    }

    pub fn fragment_shader(&self) -> String {
        self.fragment_shader.clone()
    }

    pub fn vertex_shader(&self) -> String {
        self.vertex_shader.clone()
    }

    pub fn zoom_enabled(&self) -> bool {
        self.zoom_enabled
    }

    pub fn shader_enabled(&self) -> bool {
        self.shader_enabled
    }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    pub fn debug_enabled(&self) -> bool {
        self.debug_overlay_enabled
    }

    pub fn ui_mode(&self) -> UiMode {
        self.ui_mode
    }

    pub fn switch_ui_mode(&mut self) {
        self.ui_mode.next()
    }

    pub fn target_pixel(&self) -> Option<Pixel> {
        self.target_pixel
    }

    pub fn set_target_pixel(&mut self, target_pixel: Pixel) {
        self.target_pixel = Some(target_pixel)
    }

    pub fn clear_target_pixel(&mut self) {
        self.target_pixel = None
    }

    pub fn camera_position(&self) -> Vec2 {
        self.camera_position
    }

    pub fn set_camera_position(&mut self, position: Vec2) {
        self.camera_position = position
    }

    pub fn reset_camera_position(&mut self) {
        self.camera_position = Vec2::new(RENDER_SIZE.0 as f32 / 2.0, RENDER_SIZE.1 as f32 / 2.0);
    }

    pub fn toggle_debug(&mut self) {
        self.debug_overlay_enabled = !self.debug_overlay_enabled;
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
        brush: &mut Brush,
        mouse_world_position: Vec2,
        render_ratio: (f32, f32),
        seed: u64,
    ) {
        self.draw_toolbar(render_ratio, brush);
        if self.debug_overlay_enabled {
            self.draw_debug(chunk_grid, brush, mouse_world_position, seed)
        } else {
            self.draw_info(brush);
        }
    }

    /// This function is called internally by UserInterface.draw()
    /// This should probably not be called directly
    fn draw_debug(
        &mut self,
        chunk_grid: &mut ChunkGrid,
        brush: &mut Brush,
        mouse_world_position: Vec2,
        seed: u64,
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

                if ui.button(None, "Empty pixelgrid") {
                    chunk_grid.clear();
                }
                if ui.button(None, "Regenerate pixelgrid") {
                    chunk_grid.regenerate_chunks(seed);
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

    pub fn draw_toolbar(&mut self, render_ratio: (f32, f32), brush: &mut Brush) {
        match self.ui_mode {
            UiMode::Draw => {
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
                    .titlebar(false)
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
            UiMode::Select => (),
        }
    }
    pub fn draw_info(&mut self, brush: &mut Brush) {
        match self.ui_mode {
            UiMode::Draw => {
                widgets::Window::new(hash!(), vec2(0.0, 0.0), vec2(200.0, 150.0))
                    .label("Draw information")
                    .movable(true)
                    .titlebar(false)
                    .ui(&mut *root_ui(), |ui| {
                        // Brush information
                        ui.separator();

                        ui.label(
                            None,
                            format!("Selected pixel: {}", brush.pixel_type().name_str()).as_str(),
                        );
                        ui.label(
                            None,
                            format!("Selected brush type: {}", brush.brush_type().as_str())
                                .as_str(),
                        );
                        ui.label(None, format!("Brush size: {}", brush.size()).as_str());
                    });
            }

            UiMode::Select => {
                widgets::Window::new(hash!(), vec2(0.0, 0.0), vec2(200.0, 150.0))
                    .label("Select information")
                    .movable(true)
                    .titlebar(false)
                    .ui(&mut *root_ui(), |ui| {
                        // Brush information
                        ui.separator();

                        if let Some(pixel) = self.target_pixel() {
                            ui.label(
                                None,
                                format!("Targeted pixel: {}", pixel.pixel_type().name_str())
                                    .as_str(),
                            );
                        } else {
                            ui.label(None, format!("Targeted pixel: {}", "None").as_str());
                        }
                        ui.label(
                            None,
                            format!("Camera position: {}", self.camera_position()).as_str(),
                        );
                    });
            }
        }
    }
}
#[derive(Copy, Clone, PartialEq)]
pub enum UiMode {
    Select,
    Draw,
}
impl UiMode {
    pub fn next(&mut self) {
        let idx = UI_MODE_ORDER.iter().position(|p| p == self).unwrap();
        *self = UI_MODE_ORDER[(idx + 1) % UI_MODE_ORDER.len()];
    }

    pub fn previous(&mut self) {
        let idx = UI_MODE_ORDER.iter().position(|p| p == self).unwrap();
        *self = UI_MODE_ORDER[(idx + UI_MODE_ORDER.len() - 1) % UI_MODE_ORDER.len()];
    }
}
const UI_MODE_ORDER: &[UiMode] = &[UiMode::Select, UiMode::Draw];
