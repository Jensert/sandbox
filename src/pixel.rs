use crate::pixel_grid::{GridMovement, PixelGrid};
use macroquad::{prelude::*, rand::RandGenerator};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PixelType {
    Sand,
    Water,
}
impl PixelType {
    pub fn next(&mut self) {
        match *self {
            PixelType::Sand => *self = PixelType::Water,
            PixelType::Water => *self = PixelType::Sand,
        }
    }
    pub fn previous(&mut self) {
        match *self {
            PixelType::Sand => *self = PixelType::Water,
            PixelType::Water => *self = PixelType::Sand,
        }
    }

    pub fn get(&self) -> &str {
        match self {
            PixelType::Sand => "Sand",
            PixelType::Water => "Water",
        }
    }

    pub fn update(
        &self,
        pixel_grid: &PixelGrid,
        x: u32,
        y: u32,
        rng: &RandGenerator,
    ) -> Option<GridMovement> {
        match self {
            PixelType::Sand => update_sand(pixel_grid, x, y, rng),
            PixelType::Water => update_water(pixel_grid, x, y, rng),
        }
    }

    /// Returns a boolean indicating whether the pixel was correctly updated or not
    /// It was updated if the space below the current position was free
    /// if it was updated you should probably skip any oother updates in that same frame
    pub fn apply_gravity(&self, pixel_grid: &PixelGrid, grid_movement: &mut GridMovement) -> bool {
        let check_position = (
            grid_movement.old_position.0,
            grid_movement.old_position.1 + 1,
        );
        if pixel_grid.get(check_position).is_free() {
            grid_movement.new_position.1 = check_position.1;
            return true;
        }
        false
    }

    pub fn fall(
        &self,
        pixel_grid: &PixelGrid,
        grid_movement: &mut GridMovement,
        direction: i8,
    ) -> bool {
        if direction == 0 {
            // If there is a sand pixel below, check 1 down and right
            let check_position = (
                grid_movement.old_position.0 + 1,
                grid_movement.old_position.1 + 1,
            );
            if pixel_grid.get(check_position).is_free() {
                grid_movement.new_position = check_position;
                return true;
            }

            // If there is a sand pixel below, check 1 down and left
            let check_position = (
                grid_movement.old_position.0 - 1,
                grid_movement.old_position.1 + 1,
            );
            if pixel_grid.get(check_position).is_free() {
                grid_movement.new_position = check_position;
                return true;
            }
        } else {
            // If there is a sand pixel below, check 1 down and left
            let check_position = (
                grid_movement.old_position.0 - 1,
                grid_movement.old_position.1 + 1,
            );
            if pixel_grid.get(check_position).is_free() {
                grid_movement.new_position = check_position;
                return true;
            }

            // If there is a sand pixel below, check 1 down and right
            let check_position = (
                grid_movement.old_position.0 + 1,
                grid_movement.old_position.1 + 1,
            );
            if pixel_grid.get(check_position).is_free() {
                grid_movement.new_position = check_position;
                return true;
            }
        }
        return false;
    }

    pub fn settle(
        &self,
        pixel_grid: &PixelGrid,
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
            if pixel_grid.get(check_position).is_free() {
                grid_movement.new_position = check_position;
                return true;
            }

            // If there is a water pixel below, check 1 down and left
            let check_position = (
                grid_movement.old_position.0 - 1,
                grid_movement.old_position.1,
            );
            if pixel_grid.get(check_position).is_free() {
                grid_movement.new_position = check_position;
                return true;
            }
        } else {
            // If there is a water pixel below, check 1 down and left
            let check_position = (
                grid_movement.old_position.0 - 1,
                grid_movement.old_position.1,
            );
            if pixel_grid.get(check_position).is_free() {
                grid_movement.new_position = check_position;
                return true;
            }

            // If there is a water pixel below, check 1 down and right
            let check_position = (
                grid_movement.old_position.0 + 1,
                grid_movement.old_position.1,
            );
            if pixel_grid.get(check_position).is_free() {
                grid_movement.new_position = check_position;
                return true;
            }
        }
        return false;
    }
}

pub fn draw_pixel(pixel_type: PixelType, x: u32, y: u32) {
    let x = x as f32;
    let y = y as f32;
    let w = 1.0;
    let h = 1.0;
    match pixel_type {
        PixelType::Sand => draw_rectangle(x, y, w, h, BROWN),
        PixelType::Water => draw_rectangle(x, y, w, h, BLUE),
    }
}

pub fn update_sand(
    pixel_grid: &PixelGrid,
    x: u32,
    y: u32,
    rng: &RandGenerator,
) -> Option<GridMovement> {
    let old_position = (x, y);
    let mut new_position = old_position; // new position to be returned
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
    pixel_grid: &PixelGrid,
    x: u32,
    y: u32,
    rng: &RandGenerator,
) -> Option<GridMovement> {
    // Check if y position is bottom of the screen
    // Check if there is no water pixel below current position
    let old_position = (x, y);
    let mut new_position = (x, y + 1); // If it isn't set new pos 1 pixel down
    let pixel_type = PixelType::Water;
    if pixel_grid.get(new_position).is_free() {
        let res = GridMovement::new(old_position, new_position, pixel_type); // return the old pos, new pos and the pixel data
        return Some(res);
    }

    // If there is a water pixel below, check 1 down and right
    new_position = (x + 1, y + 1);
    if pixel_grid.get(new_position).is_free() {
        let res = GridMovement::new(old_position, new_position, pixel_type); // return the old pos, new pos and the pixel data
        return Some(res);
    }

    // If there is a water pixel below, check 1 down and left
    new_position = (x - 1, y + 1);
    if pixel_grid.get(new_position).is_free() {
        let res = GridMovement::new(old_position, new_position, pixel_type); // return the old pos, new pos and the pixel data
        return Some(res);
    }

    // Finally check directly left and right

    // If there is a water pixel below, check 1 down and right
    new_position = (x + 1, y);
    if pixel_grid.get(new_position).is_free() {
        let res = GridMovement::new(old_position, new_position, pixel_type); // return the old pos, new pos and the pixel data
        return Some(res);
    }

    // If there is a water pixel below, check 1 down and left
    new_position = (x - 1, y);
    if pixel_grid.get(new_position).is_free() {
        let res = GridMovement::new(old_position, new_position, pixel_type); // return the old pos, new pos and the pixel data
        return Some(res);
    }

    return None;
}
