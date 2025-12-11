use crate::pixelgrid::{Chunk, GridMovement};
use crate::pixeltype::{AIR, PixelType};
use macroquad::{prelude::*, rand::RandGenerator};

#[derive(Clone, Copy)]
pub struct Pixel {
    pixel_type: PixelType,
    color: Color,
    temperature: i32,
    movement_speed: f32,
}
impl Pixel {
    pub fn empty() -> Self {
        Pixel {
            pixel_type: PixelType::Air,
            color: AIR,
            temperature: 1,
            movement_speed: PixelType::Air.movement_speed(),
        }
    }
    pub fn from_pixel_type(pixel_type: PixelType, rng: &RandGenerator) -> Self {
        let color = pixel_type.to_color_shade(rng);
        let movement_speed = pixel_type.movement_speed();
        Self {
            pixel_type,
            color,
            temperature: 1,
            movement_speed,
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
            PixelType::Stone => update_stone(pixel_grid, x, y, rng),
            PixelType::Dirt => update_dirt(pixel_grid, x, y, rng),
            PixelType::Grass => update_grass(pixel_grid, x, y, rng),
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

    if rng.gen_range(0.0, 1.0) > pixel_type.movement_speed() {
        return None;
    }

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

pub fn update_stone(
    pixel_grid: &Chunk,
    x: i32,
    y: i32,
    rng: &RandGenerator,
) -> Option<GridMovement> {
    let old_position = (x, y);
    let new_position = old_position;
    let pixel_type = PixelType::Stone;

    if rng.gen_range(0.0, 1.0) > pixel_type.movement_speed() {
        return None;
    }

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

pub fn update_dirt(
    pixel_grid: &Chunk,
    x: i32,
    y: i32,
    rng: &RandGenerator,
) -> Option<GridMovement> {
    let old_position = (x, y);
    let new_position = old_position;
    let pixel_type = PixelType::Dirt;

    if rng.gen_range(0.0, 1.0) > pixel_type.movement_speed() {
        return None;
    }

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

pub fn update_grass(
    pixel_grid: &Chunk,
    x: i32,
    y: i32,
    rng: &RandGenerator,
) -> Option<GridMovement> {
    let old_position = (x, y);
    let new_position = old_position;
    let pixel_type = PixelType::Grass;

    if rng.gen_range(0.0, 1.0) > pixel_type.movement_speed() {
        return None;
    }

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

    if rng.gen_range(0.0, 1.0) > pixel_type.movement_speed() {
        return None;
    }

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

    if rng.gen_range(0.0, 1.0) > pixel_type.movement_speed() {
        return None;
    }

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
