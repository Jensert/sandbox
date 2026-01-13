use macroquad::{
    color::{PURPLE, WHITE},
    input::{KeyCode, is_key_down},
    math::Vec2,
    shapes::draw_rectangle,
    texture::{DrawTextureParams, Texture2D, draw_texture_ex, load_texture},
};

pub struct Dwarf {
    movement_speed: f32,
    direction: Vec2,
    velocity: Vec2,
    position: Vec2,
    sprite: Texture2D,
}

impl Dwarf {
    pub async fn new() -> Self {
        Self {
            movement_speed: 1.0,
            direction: Vec2::ZERO,
            velocity: Vec2::ZERO,
            position: Vec2::ZERO,
            sprite: load_texture("textures/minion2.png")
                .await
                .expect("expected minion texture png at path"),
        }
    }

    pub fn move_player(&mut self) {
        let mut dir_x = 0.0;
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            dir_x -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            dir_x += 1.0;
        }

        self.direction = Vec2::new(dir_x, 0.0);

        self.velocity.x = self.direction.x * self.movement_speed;

        self.position += self.velocity;
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.sprite,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(30.0, 30.0)),
                ..Default::default()
            },
        );
    }
}
