use crate::pixel::{PixelType, draw_pixel, update_sand, update_water};
use macroquad::prelude::*;
use std::collections::HashMap;

pub struct PixelGrid {
    width: u32,
    height: u32,
    grid: HashMap<(u32, u32), PixelType>,
}
impl PixelGrid {
    pub fn new(size: (u32, u32)) -> Self {
        Self {
            width: size.0,
            height: size.1,
            grid: HashMap::new(),
        }
    }
    pub fn grid(&self) -> &HashMap<(u32, u32), PixelType> {
        return &self.grid;
    }

    pub fn grid_mut(&mut self) -> &mut HashMap<(u32, u32), PixelType> {
        return &mut self.grid;
    }

    pub fn update(&mut self) {
        //////////////////(Old X, Y)  (New X, Y)  Pixel to move
        let changes: Vec<((u32, u32), (u32, u32), PixelType)> = self
            .grid
            .iter() // Iterate over the hashmap
            // Filter certain keys and map them
            .filter_map(|(&(x, y), &pixel_type)| {
                // Match the pixel type to determine which behaviour to apply
                match pixel_type {
                    PixelType::Sand => {
                        let res = update_sand(&self, x, y);
                        return res;
                    }
                    PixelType::Water => {
                        let res = update_water(&self, x, y);
                        return res;
                    }
                }
            })
            .collect();
        // Here we loop over the changes vector and apply all modifications in the grid hashmap
        for (old_pos, new_pos, pixel) in changes {
            self.grid.remove(&old_pos); // First we remove the pixel from the key at the old position
            self.grid.insert(new_pos, pixel); // Then we insert that pixel into a new key
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

    pub fn get(&self, pos: (u32, u32)) -> Option<&PixelType> {
        return self.grid.get(&pos);
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
