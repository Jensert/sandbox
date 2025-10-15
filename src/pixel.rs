use crate::pixel_grid::{Chunk, GridMovement};
use macroquad::{prelude::*, rand::RandGenerator};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PixelType {
    Sand,
    Water,
    Air,
    Dirt,
    Stone,
    Grass,
}
impl PixelType {
    pub fn next(&mut self) {
        match *self {
            PixelType::Sand => *self = PixelType::Water,
            PixelType::Water => *self = PixelType::Dirt,
            PixelType::Dirt => *self = PixelType::Stone,
            PixelType::Stone => *self = PixelType::Grass,
            PixelType::Grass => *self = PixelType::Sand,
            _ => (),
        }
    }
    pub fn previous(&mut self) {
        match *self {
            PixelType::Sand => *self = PixelType::Grass,
            PixelType::Grass => *self = PixelType::Stone,
            PixelType::Stone => *self = PixelType::Dirt,
            PixelType::Dirt => *self = PixelType::Water,
            PixelType::Water => *self = PixelType::Sand,
            _ => (),
        }
    }

    pub fn get(&self) -> &str {
        match self {
            PixelType::Sand => "Sand",
            PixelType::Water => "Water",
            PixelType::Dirt => "Dirt",
            PixelType::Stone => "Stone",
            PixelType::Grass => "Grass",
            PixelType::Air => "Air",
        }
    }

    pub fn to_color(&self) -> Color {
        match self {
            PixelType::Sand => BEIGE,
            PixelType::Water => BLUE,
            PixelType::Dirt => DARKBROWN,
            PixelType::Stone => GRAY,
            PixelType::Grass => DARKGREEN,
            PixelType::Air => Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
        }
    }

    pub fn update(
        &self,
        chunk: &Chunk,
        x: i32,
        y: i32,
        rng: &RandGenerator,
    ) -> Option<GridMovement> {
        match self {
            PixelType::Sand => update_sand(chunk, x, y, rng),
            PixelType::Water => update_water(chunk, x, y, rng),
            _ => None,
        }
    }

    /// Returns a boolean indicating whether the pixel was correctly updated or not
    /// It was updated if the space below the current position was free
    /// if it was updated you should probably skip any oother updates in that same frame
    pub fn apply_gravity(&self, pixel_grid: &Chunk, grid_movement: &mut GridMovement) -> bool {
        let check_position = (
            grid_movement.old_position.0,
            grid_movement.old_position.1 + 1,
        );
        if pixel_grid
            .query(check_position.0, check_position.1)
            .is_free()
        {
            grid_movement.new_position.1 = check_position.1;
            return true;
        }
        false
    }

    pub fn fall(
        &self,
        pixel_grid: &Chunk,
        grid_movement: &mut GridMovement,
        direction: i8,
    ) -> bool {
        if direction == 0 {
            // If there is a sand pixel below, check 1 down and right
            let check_position = (
                grid_movement.old_position.0 + 1,
                grid_movement.old_position.1 + 1,
            );
            if pixel_grid
                .query(check_position.0, check_position.1)
                .is_free()
            {
                grid_movement.new_position = check_position;
                return true;
            }

            // If there is a sand pixel below, check 1 down and left
            let check_position = (
                grid_movement.old_position.0 - 1,
                grid_movement.old_position.1 + 1,
            );
            if pixel_grid
                .query(check_position.0, check_position.1)
                .is_free()
            {
                grid_movement.new_position = check_position;
                return true;
            }
        } else {
            // If there is a sand pixel below, check 1 down and left
            let check_position = (
                grid_movement.old_position.0 - 1,
                grid_movement.old_position.1 + 1,
            );
            if pixel_grid
                .query(check_position.0, check_position.1)
                .is_free()
            {
                grid_movement.new_position = check_position;
                return true;
            }

            // If there is a sand pixel below, check 1 down and right
            let check_position = (
                grid_movement.old_position.0 + 1,
                grid_movement.old_position.1 + 1,
            );
            if pixel_grid
                .query(check_position.0, check_position.1)
                .is_free()
            {
                grid_movement.new_position = check_position;
                return true;
            }
        }
        return false;
    }

    pub fn settle(
        &self,
        pixel_grid: &Chunk,
        grid_movement: &mut GridMovement,
        direction: i8,
    ) -> bool {
        // Check random direction to see if it should first move left or right
        if direction == 0 {
            // If there is a water pixel below, check 1 down and right
            let check_position = (
                grid_movement.old_position.0 + 1,
                grid_movement.old_position.1,
            );
            if pixel_grid
                .query(check_position.0, check_position.1)
                .is_free()
            {
                grid_movement.new_position = check_position;
                return true;
            }

            // If there is a water pixel below, check 1 down and left
            let check_position = (
                grid_movement.old_position.0 - 1,
                grid_movement.old_position.1,
            );
            if pixel_grid
                .query(check_position.0, check_position.1)
                .is_free()
            {
                grid_movement.new_position = check_position;
                return true;
            }
        } else {
            // If there is a water pixel below, check 1 down and left
            let check_position = (
                grid_movement.old_position.0 - 1,
                grid_movement.old_position.1,
            );
            if pixel_grid
                .query(check_position.0, check_position.1)
                .is_free()
            {
                grid_movement.new_position = check_position;
                return true;
            }

            // If there is a water pixel below, check 1 down and right
            let check_position = (
                grid_movement.old_position.0 + 1,
                grid_movement.old_position.1,
            );
            if pixel_grid
                .query(check_position.0, check_position.1)
                .is_free()
            {
                grid_movement.new_position = check_position;
                return true;
            }
        }
        return false;
    }
}

pub fn draw_pixel(pixel_type: PixelType, x: i32, y: i32) {
    let x = x as f32;
    let y = y as f32;
    let w = 1.0;
    let h = 1.0;
    match pixel_type {
        PixelType::Sand => draw_rectangle(x, y, w, h, BEIGE),
        PixelType::Water => draw_rectangle(x, y, w, h, BLUE),
        PixelType::Dirt => draw_rectangle(x, y, w, h, DARKBROWN),
        PixelType::Stone => draw_rectangle(x, y, w, h, GRAY),
        PixelType::Grass => draw_rectangle(x, y, w, h, DARKGREEN),
        PixelType::Air => draw_rectangle(
            x,
            y,
            w,
            h,
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
        ),
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
