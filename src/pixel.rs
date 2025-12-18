use std::cmp::max;

use crate::RENDER_SIZE;
use crate::pixelgrid::{Chunk, GridMovement, GridQuery};
use crate::pixeltype::{AIR, PixelMatter, PixelState, PixelType};
use macroquad::{prelude::*, rand::RandGenerator};

#[derive(Clone, Copy)]
pub struct Pixel {
    pixel_type: PixelType,
    color: Color,
    temperature: i32,
    stability: f32,
    state: PixelState,
}
impl Pixel {
    pub fn empty() -> Self {
        Pixel {
            pixel_type: PixelType::Air,
            color: AIR,
            temperature: 1,
            stability: 1.0,
            state: PixelState::Stable,
        }
    }
    pub fn from_pixel_type(pixel_type: PixelType, rng: &RandGenerator) -> Self {
        let color = pixel_type.to_color_shade(rng);
        Self {
            pixel_type,
            color,
            temperature: 1,
            stability: 0.0,
            state: PixelState::Falling,
        }
    }

    pub fn pixel_type(&self) -> PixelType {
        self.pixel_type
    }
    pub fn movement_speed(&self) -> f32 {
        self.pixel_type.movement_speed()
    }
    pub fn matter(&self) -> PixelMatter {
        self.pixel_type.matter()
    }
    pub fn stability_decay(&self) -> f32 {
        self.pixel_type.stability_decay()
    }
    pub fn provides_support(&self) -> bool {
        self.matter() == PixelMatter::Solid && self.state == PixelState::Stable
    }

    pub fn set_stability(&mut self, new_stability: f32) {
        self.stability = new_stability;
    }
    pub fn set_state(&mut self, new_state: PixelState) {
        self.state = new_state;
    }

    pub fn calculate_stability(&self, pixel_grid: &Chunk, x: i32, y: i32) -> (f32, PixelState) {
        // If self is not solid or if self state is falling then stability = 0.0
        if self.matter() != PixelMatter::Solid {
            return (0.0, PixelState::Falling);
        }

        // If floor of map is reached, then always return stable
        if y == RENDER_SIZE.1 as i32 - 1 {
            return (1.0, PixelState::Stable);
        }

        let query_below = pixel_grid.query(x, y + 1);

        // If pixel below is solid then stability = 1.0
        if let Some(p_below) = pixel_grid.query(x, y + 1).is_solid() {
            if p_below.provides_support() {
                return (1.0, PixelState::Stable);
            }
        }
        // get left and right neighbouring stabilities
        let pixel_left = pixel_grid
            .query(x - 1, y)
            .is_solid()
            .map(|p| p.stability)
            .unwrap_or(0.0);
        let pixel_right = pixel_grid
            .query(x + 1, y)
            .is_solid()
            .map(|p| p.stability)
            .unwrap_or(0.0);

        let max_neighbour_stability = pixel_left.max(pixel_right); // Get maximum of the neighbouring stabilities
        let stability = clamp(max_neighbour_stability - self.stability_decay(), 0.0, 1.0); // return value is new stability between 0.0 or 1.0

        let state = if stability > 0.0 {
            PixelState::Stable
        } else {
            PixelState::Falling
        };

        return (stability, state);
    }

    pub fn update_new(
        &self,
        pixel_grid: &Chunk,
        x: i32,
        y: i32,
        rng: &RandGenerator,
    ) -> Option<GridMovement> {
        if self.stability > 0.0 {
            return None;
        }

        if rng.gen_range(0.0, 1.0) > self.movement_speed() {
            return None;
        }

        match self.matter() {
            PixelMatter::Liquid => {
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

            PixelMatter::Solid => {
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
