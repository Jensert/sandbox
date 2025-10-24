use crate::pixelgrid::{Chunk, GridMovement};
use crate::pixeltype::PixelType;
use macroquad::{prelude::*, rand::RandGenerator};

pub struct Pixel {
    pixeltype: PixelType,
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

    let mut grid_movement = GridMovement::new(old_position, new_position, pixel_type);

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

    let mut grid_movement = GridMovement::new(old_position, new_position, pixel_type);

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
