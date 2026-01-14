use macroquad::color::Color;

pub const LAVA: Color = Color::new(0.70, 0.16, 0.22, 1.00);
pub const STONE: Color = Color::new(0.51, 0.51, 0.51, 1.00);
pub const DEEPSTONE: Color = Color::new(0.31, 0.31, 0.31, 1.00);
pub const GRASS: Color = Color::new(0.00, 0.46, 0.17, 1.00);
pub const WATER: Color = Color::new(0.00, 0.47, 0.95, 1.00);
pub const AIR: Color = Color::new(0.40, 0.75, 1.00, 1.00);
pub const SAND: Color = Color::new(0.83, 0.69, 0.51, 1.00);
pub const DIRT: Color = Color::new(0.30, 0.25, 0.18, 1.00);
pub const PURPLE: Color = Color::new(0.78, 0.48, 1.00, 1.00);

use crate::{
    pixel::Pixel,
    pixelgrid::{Chunk, GridMovement},
};

const PIXEL_ORDER: &[PixelType] = &[
    PixelType::Sand,
    PixelType::Water,
    PixelType::Lava,
    PixelType::Dirt,
    PixelType::Stone,
    PixelType::HardStone,
    PixelType::Grass,
    PixelType::Minion,
];

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PixelType {
    Air,

    Sand,
    Water,
    Lava,
    Dirt,
    Stone,
    HardStone,
    Grass,

    Minion,
}
impl PixelType {
    pub fn count() -> usize {
        return PIXEL_ORDER.len();
    }

    pub fn first() -> PixelType {
        *PIXEL_ORDER.first().expect("Expected a PixelType")
    }

    pub fn next(&mut self) {
        let idx = PIXEL_ORDER.iter().position(|p| p == self).unwrap();
        *self = PIXEL_ORDER[(idx + 1) % PIXEL_ORDER.len()];
    }

    pub fn previous(&mut self) {
        let idx = PIXEL_ORDER.iter().position(|p| p == self).unwrap();
        *self = PIXEL_ORDER[(idx + PIXEL_ORDER.len() - 1) % PIXEL_ORDER.len()];
    }
    pub fn name_str(&self) -> &str {
        match self {
            PixelType::Air => "Air",

            PixelType::Sand => "Sand",
            PixelType::Water => "Water",
            PixelType::Lava => "Lava",
            PixelType::Dirt => "Dirt",
            PixelType::Stone => "Stone",
            PixelType::HardStone => "Hard Stone",
            PixelType::Grass => "Grass",

            PixelType::Minion => "Minion",
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
    pub fn color(&self) -> Color {
        match self {
            PixelType::Sand => SAND, //Color::new(rng.gen_range(0.7, 0.9), 0.69, 0.51, 1.00), // Beige
            PixelType::Water => WATER, //Color::new(0.00, 0.47, rng.gen_range(0.89, 0.99), 1.00), // Blue
            PixelType::Lava => LAVA, //Color::new(rng.gen_range(0.65, 0.75), 0.25, 0.05, 1.00), // Red
            PixelType::Dirt => DIRT, //Color::new(rng.gen_range(0.25, 0.35), 0.25, 0.18, 1.00), // Darkbrown
            PixelType::Stone => STONE, //Color::new(0.5, 0.5, rng.gen_range(0.48, 0.53), 1.00), // Gray
            PixelType::HardStone => DEEPSTONE, //Color::new(0.19, 0.20, rng.gen_range(0.18, 0.33), 1.00), // Dark Gray
            PixelType::Grass => GRASS, //Color::new(0.00, rng.gen_range(0.38, 0.51), 0.17, 1.00), // Dark green
            PixelType::Air => Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
            PixelType::Minion => PURPLE,
        }
    }

    pub fn movement_speed(&self) -> f32 {
        match self {
            PixelType::Sand => 1.0, //Color::new(rng.gen_range(0.7, 0.9), 0.69, 0.51, 1.00), // Beige
            PixelType::Water => 1.0, //Color::new(0.00, 0.47, rng.gen_range(0.89, 0.99), 1.00), // Blue
            PixelType::Lava => 0.25, //Color::new(rng.gen_range(0.65, 0.75), 0.25, 0.05, 1.00), // Red
            PixelType::Dirt => 0.5, //Color::new(rng.gen_range(0.25, 0.35), 0.25, 0.18, 1.00), // Darkbrown
            PixelType::Stone => 0.1, //Color::new(0.5, 0.5, rng.gen_range(0.48, 0.53), 1.00), // Gray
            PixelType::HardStone => 0.0, //Color::new(0.19, 0.20, rng.gen_range(0.18, 0.33), 1.00), // Dark Gray
            PixelType::Grass => 0.3, //Color::new(0.00, rng.gen_range(0.38, 0.51), 0.17, 1.00), // Dark green
            PixelType::Air => 0.0,
            PixelType::Minion => 1.0,
        }
    }

    pub fn stability_decay(&self) -> f32 {
        match self {
            PixelType::Sand => 0.5, //Color::new(rng.gen_range(0.7, 0.9), 0.69, 0.51, 1.00), // Beige
            PixelType::Water => 1.0, //Color::new(0.00, 0.47, rng.gen_range(0.89, 0.99), 1.00), // Blue
            PixelType::Lava => 1.0, //Color::new(rng.gen_range(0.65, 0.75), 0.25, 0.05, 1.00), // Red
            PixelType::Dirt => 0.2, //Color::new(rng.gen_range(0.25, 0.35), 0.25, 0.18, 1.00), // Darkbrown
            PixelType::Stone => 0.1, //Color::new(0.5, 0.5, rng.gen_range(0.48, 0.53), 1.00), // Gray
            PixelType::HardStone => 0.0, //Color::new(0.19, 0.20, rng.gen_range(0.18, 0.33), 1.00), // Dark Gray
            PixelType::Grass => 0.18, //Color::new(0.00, rng.gen_range(0.38, 0.51), 0.17, 1.00), // Dark green
            PixelType::Air => 1.0,
            PixelType::Minion => 1.0,
        }
    }

    pub fn to_pixel(&self) -> Pixel {
        Pixel::from_pixel_type(*self)
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

    pub fn matter(&self) -> PixelMatter {
        match self {
            PixelType::Air => PixelMatter::Gas,
            PixelType::Sand => PixelMatter::Solid,
            PixelType::Water => PixelMatter::Liquid,
            PixelType::Lava => PixelMatter::Liquid,
            PixelType::Dirt => PixelMatter::Solid,
            PixelType::Stone => PixelMatter::Solid,
            PixelType::HardStone => PixelMatter::Solid,
            PixelType::Grass => PixelMatter::Solid,
            PixelType::Minion => PixelMatter::Minion,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum PixelMatter {
    Solid,
    Liquid,
    Gas,
    Minion,
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PixelState {
    Stable,
    Falling,
}
