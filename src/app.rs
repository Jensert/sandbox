use std::fs::read_to_string;

use macroquad::{prelude::*, rand::RandGenerator};

use crate::{
    FIXED_TIMESTEP, RENDER_SIZE, brush::Brush, mapgenerator::MapGenerator, pixelgrid::ChunkGrid,
    ui::UserInterface,
};

pub struct App {
    render_ratio: (f32, f32),

    render_target: RenderTarget,

    vertex_shader: String,
    fragment_shader: String,
    shader: Material,

    render_camera: Camera2D,
    default_camera: Camera2D,

    mouse_world_position: Vec2,
    should_quit: bool,
    total_scroll: f32,

    app_timer: AppTimer,
    chunk_grid: ChunkGrid,
    map_generator: MapGenerator,
    brush: Brush,
    user_interface: UserInterface,
}
impl App {
    pub async fn new(render_ratio: (f32, f32), rng: &RandGenerator) -> Self {
        let mut chunk_grid = ChunkGrid::new(rng);
        let map_generator = MapGenerator::default();
        map_generator.generate_map(&mut chunk_grid, rng);

        println!("Loading shaders");
        let vertex_shader = read_to_string("src/vertex.glsl").expect("expected vertex glsl shader");
        let fragment_shader =
            read_to_string("src/fragment.glsl").expect("expected fragment glsl shader");

        let shader = load_material(
            ShaderSource::Glsl {
                vertex: &vertex_shader,
                fragment: &fragment_shader,
            },
            MaterialParams {
                ..Default::default()
            },
        )
        .expect("expected a proper GLSL ShaderSource");
        // Create the texture to which we will draw
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

            render_target,

            vertex_shader,
            fragment_shader,
            shader,

            render_camera,
            default_camera,

            mouse_world_position: Vec2 { x: 0.0, y: 0.0 },

            should_quit: false,
            total_scroll: 0.0,

            app_timer: AppTimer::new(),
            chunk_grid,
            map_generator,
            brush: Brush::new(),
            user_interface: UserInterface::new(),
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

    pub fn user_interface(&self) -> &UserInterface {
        &self.user_interface
    }

    pub fn user_interface_mut(&mut self) -> &mut UserInterface {
        &mut self.user_interface
    }

    pub fn app_timer(&self) -> AppTimer {
        self.app_timer
    }

    pub fn compile_shader(&mut self, vertex_shader: String, fragment_shader: String) {
        println!("Recompiling shader program");

        let shader = load_material(
            ShaderSource::Glsl {
                vertex: &vertex_shader,
                fragment: &fragment_shader,
            },
            MaterialParams {
                ..Default::default()
            },
        )
        .expect("expected a proper GLSL ShaderSource");

        self.shader = shader;
    }

    pub fn draw_ui(&mut self, rng: &RandGenerator) {
        self.user_interface.draw(
            &mut self.chunk_grid,
            &self.map_generator,
            &mut self.brush,
            self.mouse_world_position,
            &rng,
        );
    }

    pub fn render_ratio(&self) -> (f32, f32) {
        self.render_ratio
    }

    fn mouse_to_world(&self) -> Vec2 {
        let m_screen_pos = mouse_position(); // Get mouse position
        let m_world_pos = self
            .render_camera
            .screen_to_world(vec2(m_screen_pos.0, m_screen_pos.1)) // Transform mouse position to world space
            .round(); // Round world position to integer, to prevent pixels at half positions
        return m_world_pos;
    }

    fn handle_mouse_input(&mut self, rng: &RandGenerator) {
        self.mouse_world_position = self.mouse_to_world();

        let mouse_world_position = self.mouse_world_position;
        if is_mouse_button_down(MouseButton::Left) {
            self.brush()
                .draw(mouse_world_position, self.chunks_mut(), rng);
        }
        if is_mouse_button_down(MouseButton::Right) {
            self.brush().erase(mouse_world_position, self.chunks_mut());
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
        if is_key_pressed(KeyCode::GraveAccent) {
            self.user_interface.toggle_debug();
        }

        if is_key_released(KeyCode::R) {
            self.user_interface_mut().read_shader_files();
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

        let texture_vec = vec2(
            self.render_target.texture.width() * self.render_ratio.0, // We multiply the texture's dimensions by 4
            self.render_target.texture.height() * self.render_ratio.1, // Because the texture is a quarter of the size
        );
        draw_texture_ex(
            &self.render_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(texture_vec),
                flip_y: true, // Fip y is necessary because macroquad cameras with render targets flip their Y coordinates
                ..Default::default()
            },
        );

        if self.user_interface().data().shader_enabled {
            // Apply shader
            gl_use_material(&self.shader);
            draw_rectangle(0.0, texture_vec.y, texture_vec.x, -texture_vec.y, WHITE);
            gl_use_default_material();
        }
    }

    pub fn update(&mut self, rng: &RandGenerator) {
        self.chunks_mut().update(rng);
        let frag = self.user_interface().data().fragment_shader;
        let vert = self.user_interface().data().vertex_shader;
        if self.fragment_shader != frag || self.vertex_shader != vert {
            self.compile_shader(vert, frag);
        }
    }

    pub fn chunks(&self) -> &ChunkGrid {
        return &self.chunk_grid;
    }

    pub fn chunks_mut(&mut self) -> &mut ChunkGrid {
        return &mut self.chunk_grid;
    }
}
#[derive(Copy, Clone)]
pub struct AppTimer {
    accumulator: f32,
}

impl AppTimer {
    pub fn new() -> Self {
        Self { accumulator: 0.0 }
    }

    pub fn tick<F: FnMut()>(&mut self, mut physics_step: F) {
        self.accumulator += get_frame_time();

        while self.accumulator >= FIXED_TIMESTEP {
            physics_step();
            self.accumulator -= FIXED_TIMESTEP;
        }
    }
}
