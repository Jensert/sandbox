use crate::pixel::{Direction, PixelType, draw_pixel};
use macroquad::{
    prelude::*,
    rand::{ChooseRandom, RandGenerator},
};
use std::collections::HashMap;

pub struct PixelGrid {
    width: u32,
    height: u32,
    grid: HashMap<(u32, u32), PixelType>,
    last_updates: HashMap<(u32, u32), PixelType>,

    seed: u64,
    rng: RandGenerator,
}
impl PixelGrid {
    pub fn new(size: (u32, u32), seed: u64, rng: RandGenerator) -> Self {
        let mut grid = HashMap::new();
        grid.insert((0, size.1 - 1), PixelType::Ant(Direction::Right, 1));
        let last_updates = grid.clone();
        Self {
            width: size.0,
            height: size.1,
            grid,
            last_updates,
            seed,
            rng,
        }
    }
    pub fn grid(&self) -> &HashMap<(u32, u32), PixelType> {
        return &self.grid;
    }

    pub fn grid_mut(&mut self) -> &mut HashMap<(u32, u32), PixelType> {
        return &mut self.grid;
    }
    pub fn last_updates(&self) -> &HashMap<(u32, u32), PixelType> {
        return &self.last_updates;
    }

    pub fn last_updates_mut(&mut self) -> &mut HashMap<(u32, u32), PixelType> {
        return &mut self.last_updates;
    }

    pub fn reset(&mut self) {
        self.grid.clear();
        self.grid
            .insert((0, self.height - 1), PixelType::Ant(Direction::Right, 1));
        self.last_updates = self.grid.clone();
    }

    pub fn update(&mut self) {
        self.last_updates.clear();
        // We filter_map() the hashmap
        // First we match the PixelType to call the appropriate pixel update function
        // Then in each update function we check certain bounds
        // This function returns a tuple with the old position, new position and the pixeltype
        // All of the returns are saved in the changes Vector, which is then looped over again
        // to update the hashmap
        ////Returns://////(Old X, Y)  (New X, Y)  Pixel to move
        let mut changes: Vec<GridMovement> = self
            .grid()
            .iter()
            .filter_map(|(&(x, y), &pixel_type)| pixel_type.update(self, x, y, &self.rng))
            .collect();
        // Before we apply the changes we shuffle the changes vector, so that the updates are applied in random order
        // We do this to make it seem more natural and to prevent certain softlocks
        changes.shuffle();
        // Here we loop over the changes vector and apply all modifications in the grid hashmap
        // We also check if the new position is already been occupied in a previous move byh another pixel
        // We do this to prevent 2 pixels moving into the same space in 1 move, which would cause this to overwrite
        // the pixel
        for movement in changes {
            if self.last_updates.contains_key(&movement.new_position) {
                continue;
            }
            self.grid.remove(&movement.old_position); // First we remove the pixel from the key at the old position
            self.grid.insert(movement.new_position, movement.pixel_type); // Then we insert that pixel into a new key
            self.last_updates
                .insert(movement.new_position, movement.pixel_type); // And also insert it into the updated hashmap
        }
    }

    pub fn draw(&self) {
        // Here we loop over the pixel grid to draw all the pixels
        for ((x, y), pixel_type) in &self.grid {
            // Capture the position and the pixel data
            // Draw rectangle for every entgry in the hashmap, at the position it is in, with the pixel color
            draw_pixel(*pixel_type, *x, *y);
        }
    }

    pub fn get(&self, pos: (u32, u32)) -> GridQuery {
        // First check if the position is out of bounds
        if pos.1 >= self.height() || pos.1 <= 0 {
            return GridQuery::OutOfBounds;
        }
        if pos.0 >= self.width() || pos.0 < 0 {
            return GridQuery::OutOfBounds;
        }
        // If it is not out of bounds, check if there is a pixel in the position
        match self.grid().get(&pos) {
            // If there is no pixel, return GridQuery::None
            None => GridQuery::None,
            // If there is a pixel, return GridQuery::Hit(pixel data)
            Some(pixel) => GridQuery::Hit(*pixel),
        }
    }

    pub fn set(&mut self, pos: (u32, u32), pixel: PixelType) {
        self.grid.insert(pos, pixel);
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

#[derive(PartialEq)]
pub enum GridQuery {
    OutOfBounds,
    Hit(PixelType),
    None,
}
impl GridQuery {
    pub fn is_free(&self) -> bool {
        match self {
            GridQuery::OutOfBounds => false,
            GridQuery::Hit(_) => false,
            GridQuery::None => true,
        }
    }
}

pub struct GridMovement {
    pub old_position: (u32, u32),
    pub new_position: (u32, u32),
    pub pixel_type: PixelType,
}
impl GridMovement {
    pub fn new(old_position: (u32, u32), new_position: (u32, u32), pixel_type: PixelType) -> Self {
        Self {
            old_position,
            new_position,
            pixel_type,
        }
    }
}
