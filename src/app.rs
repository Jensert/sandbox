use macroquad::{prelude::*, rand::RandGenerator};

use crate::{RENDER_SIZE, brush::Brush, pixelgrid::ChunkGrid};
pub struct App {
    render_ratio: (f32, f32),

    chunk_grid: ChunkGrid,
    render_target: RenderTarget,
    render_camera: Camera2D,
    default_camera: Camera2D,

    should_quit: bool,
    total_scroll: f32,
    brush: Brush,
}
impl App {
    pub fn new(render_ratio: (f32, f32), rng: &RandGenerator) -> Self {
        let chunk_grid = ChunkGrid::new(rng);
        // Create the texture to which we will draw
        // TODO Replace with Canvas2D
        let render_target = render_target(RENDER_SIZE.0, RENDER_SIZE.1);
        // Set filter mode to nearest to prevent blurry pixels
        render_target.texture.set_filter(FilterMode::Nearest);
        // Create the camera which we use to render. The render target is attached to this camera
        let mut render_camera = Camera2D::from_display_rect(Rect {
            x: 0.0,
            y: 0.0,
            w: RENDER_SIZE.0 as f32, // this camera's viewport has the render dimensions
            h: RENDER_SIZE.1 as f32,
        });
        // Attach render target to this camera
        render_camera.render_target = Some(render_target.clone());
        // Create camera which we use to draw the final texture.
        // This camera is essentially our screen, whereas the render_camera is the viewport
        // The render_camera is then scaled to our screen dimensions during drawing
        let default_camera = Camera2D::from_display_rect(Rect {
            x: 0.0,
            y: screen_height(),
            w: screen_width(), // this camera's viewport has the screen dimensions
            h: -screen_height(),
        });
        Self {
            render_ratio,

            chunk_grid,
            render_target,
            render_camera,
            default_camera,

            should_quit: false,
            total_scroll: 0.0,

            brush: Brush::new(),
        }
    }

    pub fn running(&self) -> bool {
        !self.should_quit
    }
    fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn brush(&self) -> Brush {
        self.brush
    }
    pub fn brush_mut(&mut self) -> &mut Brush {
        &mut self.brush
    }

    pub fn render_ratio(&self) -> (f32, f32) {
        self.render_ratio
    }

    pub fn mouse_to_world(&self) -> Vec2 {
        let m_screen_pos = mouse_position(); // Get mouse position
        let m_world_pos = self
            .render_camera
            .screen_to_world(vec2(m_screen_pos.0, m_screen_pos.1)) // Transform mouse position to world space
            .round(); // Round world position to integer, to prevent pixels at half positions
        return m_world_pos;
    }
    fn handle_mouse_input(&mut self, rng: &RandGenerator) {
        if is_mouse_button_down(MouseButton::Left) {
            let world_position = self.mouse_to_world();
            self.brush().draw(world_position, self.chunks_mut(), rng);
        }

        // Handle scrolling
        // First we get the vertical scroll direction and the amount that is scrolled
        let mut scroll = mouse_wheel().1;
        // First we check if we are scrolling up
        if scroll > 0.0 {
            self.total_scroll += scroll; // Add the total amount scrolled
            // Once we scrolled 120.0 up (idk in what unit) we count it as '1 scroll'
            if self.total_scroll >= 120.0 {
                // We divide the total scroll by 120.0 to get the total scroll amount in single units
                scroll = self.total_scroll / 120.0;
                // We loop over how many times we have scrolled and do an action for every scroll
                for _ in 0..scroll as i32 {
                    if is_key_down(KeyCode::LeftShift) {
                        self.brush_mut().brush_type_mut().next();
                        continue;
                    }

                    if is_key_down(KeyCode::LeftAlt) {
                        self.brush_mut().increase_size(1.0);
                        continue;
                    }
                    self.brush_mut().pixel_type_mut().next();
                }
                self.total_scroll = 0.0;
            }
            // Then we do that exact same thing but for scrolling down
        } else if scroll < 0.0 {
            self.total_scroll += scroll;
            if self.total_scroll <= -110.0 {
                // scrolled down
                scroll = self.total_scroll / 120.0;
                for _ in 0..scroll.abs() as i32 {
                    if is_key_down(KeyCode::LeftShift) {
                        self.brush_mut().brush_type_mut().previous();
                        continue;
                    }

                    if is_key_down(KeyCode::LeftAlt) {
                        self.brush_mut().decrease_size(1.0);
                        continue;
                    }
                    self.brush_mut().pixel_type_mut().previous();
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
            self.chunk_grid.clear();
        }
    }

    pub fn handle_input(&mut self, rng: &RandGenerator) {
        self.handle_mouse_input(rng);
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
                    self.render_target.texture.width() * self.render_ratio.0, // We multiply the texture's dimensions by 4
                    self.render_target.texture.height() * self.render_ratio.1, // Because the texture is a quarter of the size
                )),
                flip_y: true,
                ..Default::default()
            },
        );
    }

    pub fn update(&mut self, rng: &RandGenerator) {
        self.chunks_mut().update(rng)
    }

    pub fn chunks(&self) -> &ChunkGrid {
        return &self.chunk_grid;
    }

    pub fn chunks_mut(&mut self) -> &mut ChunkGrid {
        return &mut self.chunk_grid;
    }
}
