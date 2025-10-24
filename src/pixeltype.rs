use macroquad::{
    color::{BEIGE, BLUE, Color, DARKBROWN, DARKGREEN, GRAY},
    rand::RandGenerator,
};

use crate::{
    pixel::{Pixel, update_sand, update_water},
    pixelgrid::{Chunk, GridMovement},
};
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

    pub fn to_str(&self) -> &str {
        match self {
            PixelType::Sand => "Sand",
            PixelType::Water => "Water",
            PixelType::Dirt => "Dirt",
            PixelType::Stone => "Stone",
            PixelType::Grass => "Grass",
            PixelType::Air => "Air",
        }
    }

    // pub fn to_color(&self) -> Color {
    //     match self {
    //         PixelType::Sand => Color::new(0.83, 0.69, 0.51, 1.00), // Beige
    //         PixelType::Water => Color::new(0.00, 0.47, 0.95, 1.00), // Blue
    //         PixelType::Dirt => Color::new(0.30, 0.25, 0.18, 1.00), // Darkbrown
    //         PixelType::Stone => Color::new(0.51, 0.51, 0.51, 1.00), // Gray
    //         PixelType::Grass => Color::new(0.00, 0.46, 0.17, 1.00), // Dark green
    //         PixelType::Air => Color {
    //             r: 0.0,
    //             g: 0.0,
    //             b: 0.0,
    //             a: 0.0,
    //         },
    //     }
    // }
    pub fn to_color_shade(&self, rng: &RandGenerator) -> Color {
        match self {
            PixelType::Sand => Color::new(rng.gen_range(0.7, 0.9), 0.69, 0.51, 1.00), // Beige
            PixelType::Water => Color::new(0.00, 0.47, rng.gen_range(0.89, 0.99), 1.00), // Blue
            PixelType::Dirt => Color::new(rng.gen_range(0.25, 0.35), 0.25, 0.18, 1.00), // Darkbrown
            PixelType::Stone => Color::new(
                rng.gen_range(0.48, 0.53),
                rng.gen_range(0.48, 0.53),
                rng.gen_range(0.48, 0.53),
                1.00,
            ), // Gray
            PixelType::Grass => Color::new(0.00, rng.gen_range(0.38, 0.51), 0.17, 1.00), // Dark green
            PixelType::Air => Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
        }
    }

    pub fn to_pixel(&self, rng: &RandGenerator) -> Pixel {
        Pixel::from_pixel_type(*self, rng)
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
