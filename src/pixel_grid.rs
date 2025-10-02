use crate::{
    CHUNK_SIZE,
    pixel::{PixelType, draw_pixel},
};
use macroquad::{
    prelude::*,
    rand::{ChooseRandom, RandGenerator},
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ChunkPosition {
    pub chunk_key: (i32, i32),
    pub chunk_coordinate: (u32, u32),
}

impl ChunkPosition {
    pub fn new(chunk_key: (i32, i32), chunk_coordinate: (u32, u32)) -> Self {
        Self {
            chunk_key,
            chunk_coordinate,
        }
    }

    pub fn from_world_position(world_position: Vec2) -> ChunkPosition {
        let (wx, wy) = (world_position.x as i32, world_position.y as i32);

        let cx = wx.div_euclid(CHUNK_SIZE.0 as i32);
        let cy = wy.div_euclid(CHUNK_SIZE.1 as i32);

        let lx = wx.rem_euclid(CHUNK_SIZE.0 as i32) as u32;
        let ly = wy.rem_euclid(CHUNK_SIZE.1 as i32) as u32;

        Self {
            chunk_key: (cx, cy),
            chunk_coordinate: (lx, ly),
        }
    }
}
pub struct ChunkGrid {
    grid: HashMap<(i32, i32), Chunk>,
    _seed: u64,
    rng: RandGenerator,
}

impl ChunkGrid {
    pub fn new(_seed: u64, rng: RandGenerator) -> Self {
        let mut grid = HashMap::new();
        grid.insert((0, 0), Chunk::new(CHUNK_SIZE, _seed));
        grid.insert((0, 1), Chunk::new(CHUNK_SIZE, _seed));
        grid.insert((1, 0), Chunk::new(CHUNK_SIZE, _seed));
        grid.insert((1, 1), Chunk::new(CHUNK_SIZE, _seed));
        Self { grid, _seed, rng }
    }

    pub fn update(&mut self) {
        for ((x, y), chunk) in self.grid.iter_mut() {
            chunk.update(&self.rng);
        }
    }

    pub fn clear(&mut self) {
        for ((_x, _y), chunk) in self.grid.iter_mut() {
            chunk.clear();
        }
    }

    pub fn get_total_pixels(&self) -> usize {
        let mut res = 0;
        for ((_, _), chunk) in self.grid.iter() {
            res += chunk.chunk.len();
        }
        res
    }

    pub fn draw(&self) {
        for ((_, _), chunk) in self.grid.iter() {
            chunk.draw();
        }
    }

    pub fn grid(&mut self) -> &mut HashMap<(i32, i32), Chunk> {
        &mut self.grid
    }

    pub fn insert_pixel(&mut self, world_position: Vec2, pixel_type: PixelType) {
        let chunk_position = ChunkPosition::from_world_position(world_position);
        self.grid
            .get_mut(&chunk_position.chunk_key)
            .unwrap()
            .chunk_mut()
            .insert(chunk_position.chunk_coordinate, pixel_type);
    }
}

pub struct Chunk {
    width: u32,
    height: u32,
    chunk: HashMap<(u32, u32), PixelType>,
    last_updates: HashMap<(u32, u32), PixelType>,

    _seed: u64,
}
impl Chunk {
    pub fn new(size: (u32, u32), _seed: u64) -> Self {
        let chunk = HashMap::new();
        let last_updates = chunk.clone();
        Self {
            width: size.0,
            height: size.1,
            chunk,
            last_updates,
            _seed,
        }
    }
    pub fn chunk(&self) -> &HashMap<(u32, u32), PixelType> {
        return &self.chunk;
    }

    pub fn chunk_mut(&mut self) -> &mut HashMap<(u32, u32), PixelType> {
        return &mut self.chunk;
    }

    pub fn update(&mut self, rng: &RandGenerator) {
        self.last_updates.clear();
        // We filter_map() the hashmap
        // First we match the PixelType to call the appropriate pixel update function
        // Then in each update function we check certain bounds
        // This function returns a tuple with the old position, new position and the pixeltype
        // All of the returns are saved in the changes Vector, which is then looped over again
        // to update the hashmap
        ////Returns://////(Old X, Y)  (New X, Y)  Pixel to move
        let mut changes: Vec<GridMovement> = self
            .chunk()
            .iter()
            .filter_map(|(&(x, y), &pixel_type)| pixel_type.update(self, x, y, &rng))
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
            self.chunk.remove(&movement.old_position); // First we remove the pixel from the key at the old position
            self.chunk
                .insert(movement.new_position, movement.pixel_type); // Then we insert that pixel into a new key
            self.last_updates
                .insert(movement.new_position, movement.pixel_type); // And also insert it into the updated hashmap
        }
    }

    pub fn draw(&self) {
        // Here we loop over the pixel grid to draw all the pixels
        for ((x, y), pixel_type) in &self.chunk {
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
        match self.chunk().get(&pos) {
            // If there is no pixel, return GridQuery::None
            None => GridQuery::None,
            // If there is a pixel, return GridQuery::Hit(pixel data)
            Some(pixel) => GridQuery::Hit(*pixel),
        }
    }

    pub fn set(&mut self, pos: (u32, u32), pixel: PixelType) {
        self.chunk.insert(pos, pixel);
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn clear(&mut self) {
        self.chunk_mut().clear();
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
