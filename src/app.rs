use macroquad::{prelude::*, rand::RandGenerator};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{pixel::PixelType, pixel_grid::PixelGrid};

pub fn window_settings() -> Conf {
    Conf {
        window_title: String::from("Sandbox"),
        window_width: 1280,
        window_height: 720,
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
    total_scroll: f32,
    selected_pixel: PixelType,
}
impl App {
    pub fn new(render_size: (u32, u32)) -> Self {
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
        println!("Started app with seed: {seed}");
        // Create pixelgrid with the seed
        let mut pixel_grid = PixelGrid::new(render_size, seed, rng);
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
        let mut default_camera = Camera2D::from_display_rect(Rect {
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
            total_scroll: 0.0,

            selected_pixel: PixelType::Sand,
        }
    }

    pub fn running(&self) -> bool {
        !self.should_quit
    }
    fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn selected_pixel(&self) -> PixelType {
        self.selected_pixel
    }

    pub fn reset_grid(&mut self) {
        self.pixels_mut().reset();
    }

    fn handle_mouse_input(&mut self) {
        if is_mouse_button_down(MouseButton::Left) {
            let m_screen_pos = mouse_position(); // Get mouse position
            let m_world_pos = self
                .render_camera
                .screen_to_world(vec2(m_screen_pos.0, m_screen_pos.1)) // Transform mouse position to world space
                .round(); // Round world position to integer, to prevent pixels at half positions
            self.pixel_grid.grid_mut().insert(
                (m_world_pos.x as u32, m_world_pos.y as u32),
                self.selected_pixel,
            );
        }

        // Scroll wheel
        let mut scroll = mouse_wheel().1;
        if scroll > 0.0 {
            self.total_scroll += scroll;
            if self.total_scroll >= 120.0 {
                // scrolled up
                scroll = self.total_scroll / 120.0;
                for _ in 0..scroll as i32 {
                    self.selected_pixel.next();
                    println!("{:?}", self.selected_pixel);
                }
                self.total_scroll = 0.0;
            }
        } else if scroll < 0.0 {
            self.total_scroll += scroll;
            if self.total_scroll <= -110.0 {
                // scrolled down
                scroll = self.total_scroll / 120.0;
                for _ in 0..scroll.abs() as i32 {
                    self.selected_pixel.previous();
                    println!("{:?}", self.selected_pixel);
                }
                self.total_scroll = 0.0;
            }
        }
    }

    fn handle_keyboard_input(&mut self) {
        if is_key_released(KeyCode::Escape) {
            self.quit();
        }
        if is_key_pressed(KeyCode::C) {
            self.pixel_grid.grid_mut().clear();
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
                    self.render_target.texture.width() * 16.0, // We multiply the texture's dimensions by 4
                    self.render_target.texture.height() * 16.0, // Because the texture is a quarter of the size
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
