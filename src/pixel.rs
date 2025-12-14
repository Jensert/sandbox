use crate::pixelgrid::{Chunk, GridMovement};
use crate::pixeltype::{AIR, PixelState, PixelType};
use macroquad::{prelude::*, rand::RandGenerator};

#[derive(Clone, Copy)]
pub struct Pixel {
    pixel_type: PixelType,
    color: Color,
    temperature: i32,
    stability: f32,
}
impl Pixel {
    pub fn empty() -> Self {
        Pixel {
            pixel_type: PixelType::Air,
            color: AIR,
            temperature: 1,
            stability: 1.0,
        }
    }
    pub fn from_pixel_type(pixel_type: PixelType, rng: &RandGenerator) -> Self {
        let color = pixel_type.to_color_shade(rng);
        Self {
            pixel_type,
            color,
            temperature: 1,
            stability: 1.0,
        }
    }

    pub fn pixel_type(&self) -> PixelType {
        self.pixel_type
    }
    pub fn movement_speed(&self) -> f32 {
        self.pixel_type.movement_speed()
    }
    pub fn solidity(&self) -> PixelState {
        self.pixel_type.state()
    }

    pub fn update_new(
        &self,
        pixel_grid: &Chunk,
        x: i32,
        y: i32,
        rng: &RandGenerator,
    ) -> Option<GridMovement> {
        if self.stability > 1.0 {
            return None;
        }

        if rng.gen_range(0.0, 1.0) > self.movement_speed() {
            return None;
        }

        match self.pixel_type.state() {
            PixelState::Liquid => {
                // Randomize horizontal preference to avoid bias
                let direction = rng.gen_range(0, 2); // 0 = left-first, 1 = right-first

                let positions_to_check = if direction == 0 {
                    [
                        (x, y + 1), // down
                        (x - 1, y), // left
                        (x + 1, y), // right
                    ]
                } else {
                    [
                        (x, y + 1), // down
                        (x + 1, y), // right
                        (x - 1, y), // left
                    ]
                };

                for (x_new, y_new) in positions_to_check {
                    if pixel_grid.query(x_new, y_new).is_free() {
                        return Some(GridMovement::new((x, y), (x_new, y_new)));
                    }
                }

                None
            }

            PixelState::Solid => {
                // Existing solid behavior (fall + diagonals)
                let direction = rng.gen_range(0, 2); // 0 = left-first, 1 = right-first

                let positions_to_check = if direction == 0 {
                    [
                        (x, y + 1),     // down
                        (x - 1, y + 1), // down-left
                        (x + 1, y + 1), // down-right
                    ]
                } else {
                    [
                        (x, y + 1),     // down
                        (x + 1, y + 1), // down-right
                        (x - 1, y + 1), // down-left
                    ]
                };

                for (x_new, y_new) in positions_to_check {
                    if pixel_grid.query(x_new, y_new).is_free() {
                        return Some(GridMovement::new((x, y), (x_new, y_new)));
                    }
                }

                None
            }

            _ => None,
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }
}
