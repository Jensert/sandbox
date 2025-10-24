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
    pub fn from_pixel_type(pixel_type: PixelType) -> Self {
        Self {
            pixel_type,
            color: WHITE,
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
            _ => None,
        }
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
    let pixel = PixelType::Sand.to_pixel();

    let mut grid_movement = GridMovement::new(old_position, new_position);

    if pixel
        .pixel_type
        .apply_gravity(pixel_grid, &mut grid_movement)
    {
        return Some(grid_movement);
    }

    let direction = rng.gen_range(0, 2);
    if pixel
        .pixel_type
        .fall(pixel_grid, &mut grid_movement, direction)
    {
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
    let pixel = PixelType::Water.to_pixel();

    let mut grid_movement = GridMovement::new(old_position, new_position);

    if pixel
        .pixel_type
        .apply_gravity(pixel_grid, &mut grid_movement)
    {
        return Some(grid_movement);
    }

    let direction = rng.gen_range(0, 2);
    if pixel
        .pixel_type
        .fall(pixel_grid, &mut grid_movement, direction)
    {
        return Some(grid_movement);
    }

    if pixel
        .pixel_type
        .settle(pixel_grid, &mut grid_movement, direction)
    {
        return Some(grid_movement);
    }

    return None;
}
