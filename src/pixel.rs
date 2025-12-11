use crate::pixelgrid::{Chunk, GridMovement};
use crate::pixeltype::PixelType;
use macroquad::{prelude::*, rand::RandGenerator};

#[derive(Clone, Copy)]
pub struct Pixel {
    pixel_type: PixelType,
    color: Color,
    temperature: i32,
}
impl Pixel {
    pub fn empty() -> Self {
        Pixel {
            pixel_type: PixelType::Air,
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
            temperature: 1,
        }
    }
    pub fn from_pixel_type(pixel_type: PixelType, rng: &RandGenerator) -> Self {
        let color = pixel_type.to_color_shade(rng);
        Self {
            pixel_type,
            color,
            temperature: 1,
        }
    }

    pub fn pixel_type(&self) -> PixelType {
        self.pixel_type
    }

    pub fn update(
        &self,
        pixel_grid: &Chunk,
        x: i32,
        y: i32,
        rng: &RandGenerator,
    ) -> Option<GridMovement> {
        match self.pixel_type {
            PixelType::Sand => update_sand(pixel_grid, x, y, rng),
            PixelType::Water => update_water(pixel_grid, x, y, rng),
            PixelType::Lava => update_lava(pixel_grid, x, y, rng),
            _ => None,
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

pub fn update_sand(
    pixel_grid: &Chunk,
    x: i32,
    y: i32,
    rng: &RandGenerator,
) -> Option<GridMovement> {
    let old_position = (x, y);
    let new_position = old_position;
    let pixel_type = PixelType::Sand;

    let mut grid_movement = GridMovement::new(old_position, new_position);

    if pixel_type.apply_gravity(pixel_grid, &mut grid_movement) {
        return Some(grid_movement);
    }

    let direction = rng.gen_range(0, 2);
    if pixel_type.fall(pixel_grid, &mut grid_movement, direction) {
        return Some(grid_movement);
    }
    return None;
}

pub fn update_water(
    pixel_grid: &Chunk,
    x: i32,
    y: i32,
    rng: &RandGenerator,
) -> Option<GridMovement> {
    let old_position = (x, y);
    let new_position = old_position;
    let pixel_type = PixelType::Water;

    let mut grid_movement = GridMovement::new(old_position, new_position);

    if pixel_type.apply_gravity(pixel_grid, &mut grid_movement) {
        return Some(grid_movement);
    }

    let direction = rng.gen_range(0, 2);
    if pixel_type.fall(pixel_grid, &mut grid_movement, direction) {
        return Some(grid_movement);
    }

    if pixel_type.settle(pixel_grid, &mut grid_movement, direction) {
        return Some(grid_movement);
    }

    return None;
}

pub fn update_lava(
    pixel_grid: &Chunk,
    x: i32,
    y: i32,
    rng: &RandGenerator,
) -> Option<GridMovement> {
    let old_position = (x, y);
    let new_position = old_position;
    let pixel_type = PixelType::Lava;

    let mut grid_movement = GridMovement::new(old_position, new_position);

    if pixel_type.apply_gravity(pixel_grid, &mut grid_movement) {
        return Some(grid_movement);
    }

    let direction = rng.gen_range(0, 2);
    if pixel_type.fall(pixel_grid, &mut grid_movement, direction) {
        return Some(grid_movement);
    }

    if pixel_type.settle(pixel_grid, &mut grid_movement, direction) {
        return Some(grid_movement);
    }

    return None;
}
