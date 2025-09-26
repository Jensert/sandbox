use std::{collections::HashMap, ops::Deref};

use macroquad::prelude::*;
pub fn window_settings() -> Conf {
    Conf {
        window_title: String::from("Sandbox"),
        window_width: 640,
        window_height: 360,
        ..Default::default()
    }
}

pub struct App {
    render_size: (u32, u32),
    pixel_grid: PixelGrid,
    render_target: RenderTarget,
    render_camera: Camera2D,
    default_camera: Camera2D,

    should_quit: bool,
}
impl App {
    pub fn new(render_size: (u32, u32)) -> Self {
        let pixel_grid = PixelGrid {
            width: render_size.0,
            height: render_size.1,
            grid: HashMap::new(),
            grid_back_buffer: HashMap::new(),
        };

        // Create the texture to which we will draw
        let render_target = render_target(render_size.0, render_size.1);
        // Set filter mode to nearest to prevent blurry pixels
        render_target.texture.set_filter(FilterMode::Nearest);
        // Create the camera which we use to render. The render target is attached to this camera
        let mut render_camera = Camera2D::from_display_rect(Rect {
            x: 0.0,
            y: 0.0,
            w: render_size.0 as f32, // this camera's viewport has the render dimensions
            h: render_size.1 as f32,
        });
        // Attach render target to this camera
        render_camera.render_target = Some(render_target.clone());
        // Create camera which we use to draw the final texture.
        // This camera is essentially our screen, whereas the render_camera is the viewport
        // The render_camera is then scaled to our screen dimensions during drawing
        let default_camera = Camera2D::from_display_rect(Rect {
            x: 0.0,
            y: 0.0,
            w: screen_width(), // this camera's viewport has the screen dimensions
            h: screen_height(),
        });
        Self {
            render_size,
            pixel_grid,
            render_target,
            render_camera,
            default_camera,

            should_quit: false,
        }
    }

    pub fn running(&self) -> bool {
        !self.should_quit
    }
    fn quit(&mut self) {
        self.should_quit = true;
    }

    fn handle_mouse_input(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let m_screen_pos = mouse_position(); // Get mouse position
            let m_world_pos = self
                .render_camera
                .screen_to_world(vec2(m_screen_pos.0, m_screen_pos.1)) // Transform mouse position to world space
                .round(); // Round world position to integer, to prevent pixels at half positions
            self.pixel_grid.grid.insert(
                (m_world_pos.x as u32, m_world_pos.y as u32),
                Pixel::new(PixelType::Sand),
            );
            println!("screen pos: {m_screen_pos:?}\nworld_pos: {m_world_pos:?}");
        }
    }

    fn handle_keyboard_input(&mut self) {
        if is_key_released(KeyCode::Escape) {
            self.quit();
        }
    }

    pub fn handle_input(&mut self) {
        self.handle_mouse_input();
        self.handle_keyboard_input();
    }

    pub fn start_drawing(&self) {
        set_camera(&self.render_camera);
    }

    pub fn stop_drawing(&self) {
        set_camera(&self.default_camera);

        draw_texture_ex(
            &self.render_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    self.render_target.texture.width() * 4.0, // We multiply the texture's dimensions by 4
                    self.render_target.texture.height() * 4.0, // Because the texture is a quarter of the size
                )), // We should change this to dynamically multiply it by the ratio: screen / render target size
                // The INITIAL sizes, not the scaled sizes after the camera projections
                ..Default::default()
            },
        );
    }

    pub fn pixels(&self) -> &PixelGrid {
        return &self.pixel_grid;
    }

    pub fn pixels_mut(&mut self) -> &mut PixelGrid {
        return &mut self.pixel_grid;
    }
}

pub struct PixelGrid {
    width: u32,
    height: u32,
    grid: HashMap<(u32, u32), Pixel>,
    grid_back_buffer: HashMap<(u32, u32), Pixel>,
}
impl PixelGrid {
    pub fn grid(&self) -> &HashMap<(u32, u32), Pixel> {
        return &self.grid;
    }

    pub fn grid_mut(&mut self) -> &mut HashMap<(u32, u32), Pixel> {
        return &mut self.grid;
    }

    pub fn update(&mut self) {
        //////////////////(Old X, Y)  (New X, Y)  Pixel to move
        let changes: Vec<((u32, u32), (u32, u32), Pixel)> = self
            .grid
            .iter() // Iterate over the hashmap
            // Filter certain keys and map them
            .filter_map(|(&(x, y), &pixel)| {
                // Match the pixel type to determine which behaviour to apply
                match pixel.pixel_type {
                    PixelType::Sand => {
                        // Check if y position is bottom of the screen
                        if y < self.height - 1 {
                            let new_pos = (x, y + 1); // If it isn't set new pos 1 pixel down
                            Some(((x, y), new_pos, pixel)) // return the old pos, new pos and the pixel data
                        } else {
                            None // If pixel is already at bottom of the screen, do nothing
                        }
                    }
                    PixelType::Water => unimplemented!(),
                }
            })
            .collect();
        // Here we loop over the changes vector and apply all modifications in the grid hashmap
        for (old_pos, new_pos, pixel) in changes {
            self.grid.remove(&old_pos); // First we remove the pixel from the key at the old position
            self.grid.insert(new_pos, pixel); // Then we insert that pixel into a new key
        }
    }

    pub fn draw(&self) {
        // Here we loop over the pixel grid to draw all the pixels
        for (pos, pixel) in &self.grid {
            // Capture the position and the pixel data
            // Draw rectangle for every entgry in the hashmap, at the position it is in, with the pixel color
            draw_rectangle(pos.0 as f32, pos.1 as f32, 1.0, 1.0, pixel.color);
        }
    }
}

#[derive(Clone, Copy)]
pub struct Pixel {
    color: Color,
    pixel_type: PixelType,
}
impl Pixel {
    pub fn new(pixel_type: PixelType) -> Self {
        let color = match pixel_type {
            PixelType::Sand => BROWN,
            PixelType::Water => BLUE,
        };

        Self { color, pixel_type }
    }

    pub fn sand() -> Self {
        Self {
            color: BROWN,
            pixel_type: PixelType::Sand,
        }
    }

    pub fn water() -> Self {
        Self {
            color: BLUE,
            pixel_type: PixelType::Water,
        }
    }
}
#[derive(Clone, Copy)]
pub enum PixelType {
    Sand,
    Water,
}
