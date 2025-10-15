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
    pub chunk_coordinate: (i32, i32),
}

impl ChunkPosition {
    pub fn new(chunk_key: (i32, i32), chunk_coordinate: (i32, i32)) -> Self {
        Self {
            chunk_key,
            chunk_coordinate,
        }
    }

    pub fn from_world_position(world_position: Vec2) -> ChunkPosition {
        let (wx, wy) = (world_position.x as i32, world_position.y as i32);

        let cx = wx.div_euclid(CHUNK_SIZE.0 as i32);
        let cy = wy.div_euclid(CHUNK_SIZE.1 as i32);

        let lx = wx.rem_euclid(CHUNK_SIZE.0 as i32);
        let ly = wy.rem_euclid(CHUNK_SIZE.1 as i32);

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
        grid.insert((0, 0), Chunk::new(CHUNK_SIZE, _seed, (0, 0)));
        grid.insert((0, 1), Chunk::new(CHUNK_SIZE, _seed, (0, 1)));
        grid.insert((1, 0), Chunk::new(CHUNK_SIZE, _seed, (1, 0)));
        grid.insert((1, 1), Chunk::new(CHUNK_SIZE, _seed, (1, 1)));
        Self { grid, _seed, rng }
    }

    pub fn update(&mut self) {
        // Updating should be multiple stages:
        // First: apply all in-chunk movements
        // Second: get all cross-chunk movements for each chunk
        // Third: apply all cross-chunk movements
        let mut cross_chunk_movements: Vec<Vec<GridMovement>> = vec![];
        for ((x, y), chunk) in self.grid.iter_mut() {
            cross_chunk_movements.push(chunk.update(&self.rng)); // Update all in-chunk movements and return all crosschunk movements
        }

        // Apply all cross chunk movements
        for chunk in cross_chunk_movements {
            for movement in chunk {
                match movement.new_chunk {
                    None => {
                        println!("chunk key not set! skipping movement");
                    }
                    Some(chunk_key) => {
                        if self.is_free(&movement) {
                            // We have to remove the pixel from the old position here

                            let old_chunk = self
                                .grid
                                .get_mut(&movement.old_chunk.unwrap())
                                .expect(format!("Expected a chunk at {:?}", chunk_key).as_str());
                            old_chunk.remove(movement.old_position.0, movement.old_position.1);
                            let chunk = self
                                .grid
                                .get_mut(&chunk_key)
                                .expect(format!("Expected a chunk at {:?}", chunk_key).as_str());
                            if chunk
                                .query(movement.new_position.0, movement.new_position.1)
                                .is_free()
                            {
                                chunk.set(
                                    movement.new_position.0,
                                    movement.new_position.1,
                                    movement.pixel_type,
                                );
                            } else {
                            }
                        }
                    }
                }
            }
        }

        // Update texture
        self.update_texture();
    }

    pub fn update_texture(&mut self) {
        for ((_, _), chunk) in self.grid.iter_mut() {
            chunk.update_texture();
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
        for ((chunk_key_x, chunk_key_y), chunk) in self.grid.iter() {
            chunk.draw(*chunk_key_x, *chunk_key_y);
        }
    }

    pub fn grid(&mut self) -> &mut HashMap<(i32, i32), Chunk> {
        &mut self.grid
    }

    pub fn set_pixel(&mut self, world_position: Vec2, pixel_type: PixelType) {
        let chunk_position = ChunkPosition::from_world_position(world_position);
        self.grid.get_mut(&chunk_position.chunk_key).unwrap().set(
            chunk_position.chunk_coordinate.0,
            chunk_position.chunk_coordinate.1,
            pixel_type,
        );
    }

    /// Check if the grid position (world position) is free, chunk-wide
    /// This requires the supplied GridMovement struct to have a chunk key
    /// and a chunk coordinate
    /// It then checks the coordinate within that chunk if it is free
    pub fn is_free(&self, grid_movement: &GridMovement) -> bool {
        match grid_movement.new_chunk {
            None => {
                println!("chunk key not set! skipping movement");
                false
            }
            Some(chunk_key) => {
                let chunk = self
                    .grid
                    .get(&chunk_key)
                    .expect(format!("Expected a chunk at {:?}", chunk_key).as_str());
                if chunk
                    .query(grid_movement.new_position.0, grid_movement.new_position.1)
                    .is_free()
                {
                    true
                } else {
                    false
                }
            }
        }
    }

    /// Does the same check as is_free(). It checks the chunk coordinate
    /// if it is free. But this returns the chunk if it is free.
    /// Can be used to directly insert into the chunk that is checked.
    pub fn get_chunk_if_free(&self, grid_movement: &GridMovement) -> Option<&Chunk> {
        match grid_movement.new_chunk {
            None => {
                println!("chunk key not set! skipping movement");
                None
            }
            Some(chunk_key) => {
                let chunk = self
                    .grid
                    .get(&chunk_key)
                    .expect(format!("Expected a chunk at {:?}", chunk_key).as_str());
                if chunk
                    .query(grid_movement.new_position.0, grid_movement.new_position.1)
                    .is_free()
                {
                    Some(&chunk)
                } else {
                    None
                }
            }
        }
    }
}

pub struct Chunk {
    width: i32,
    height: i32,
    key: (i32, i32),
    chunk: Vec<PixelType>,
    last_updates: HashMap<(i32, i32), PixelType>,

    texture: Texture2D,

    _seed: u64,
}
impl Chunk {
    pub fn new(size: (usize, usize), _seed: u64, key: (i32, i32)) -> Self {
        let chunk = vec![PixelType::Air; CHUNK_SIZE.0 as usize * CHUNK_SIZE.1 as usize];
        let last_updates = HashMap::new();

        let image = Image::gen_image_color(
            CHUNK_SIZE.0 as u16,
            CHUNK_SIZE.1 as u16,
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
        );

        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Nearest);

        Self {
            width: size.0 as i32,
            height: size.1 as i32,
            key,
            chunk,
            last_updates,

            texture,

            _seed,
        }
    }
    pub fn chunk(&self) -> &Vec<PixelType> {
        return &self.chunk;
    }

    pub fn chunk_mut(&mut self) -> &mut Vec<PixelType> {
        return &mut self.chunk;
    }

    /// The update function returns a vector of cross gridmovements. The return type is only used
    /// by the parent struct ChunkGrid to handle cross chunk movements.
    /// All mvoements in-chunk are handled by the chunk itself in their update function
    pub fn update(&mut self, rng: &RandGenerator) -> Vec<GridMovement> {
        self.last_updates.clear();
        // We filter_map() the hashmap
        // First we match the PixelType to call the appropriate pixel update function
        // Then in each update function we check certain bounds
        // This function returns a tuple with the old position, new position and the pixeltype
        // All of the returns are saved in the changes Vector, which is then looped over again
        // to update the hashmap
        ////Returns://////(Old X, Y)  (New X, Y)  Pixel to move
        let mut changes: Vec<GridMovement> = vec![];
        for y in 0..CHUNK_SIZE.1 {
            for x in 0..CHUNK_SIZE.0 {
                if let Some(pixel_type) = self.get(x as i32, y as i32) {
                    if let Some(movement) = pixel_type.update(self, x as i32, y as i32, rng) {
                        changes.push(movement);
                    }
                }
            }
        }
        // Before we apply the changes we shuffle the changes vector, so that the updates are applied in random order
        // We do this to make it seem more natural and to prevent certain softlocks
        changes.shuffle();
        // Here we loop over the changes vector and apply all modifications in the grid hashmap
        // First we check if the new position is out of bounds and should move to a different chunk
        // We also check if the new position is already been occupied in a previous move byh another pixel
        // We do this to prevent 2 pixels moving into the same space in 1 move, which would cause this to overwrite
        // the pixel
        let mut cross_chunk_movements = vec![];
        for mut movement in changes {
            // Check if the movement is out of bounds
            if movement.out_of_bounds() {
                // if it is, push to the cross_movement vector
                movement.set_chunk_keys(self.key);
                //self.chunk.remove(&movement.old_position); // First we remove the pixel from the key at the old position
                // Push the movement to the cross_chunk vector
                // which will be returned to the parent ChunkGrid
                cross_chunk_movements.push(movement);
                continue;
            }
            // Skip update if the new position is already updated this frame
            if self.last_updates.contains_key(&movement.new_position) {
                continue;
            }
            self.remove(movement.old_position.0, movement.old_position.1); // First we remove the pixel from the key at the old position
            self.set(
                movement.new_position.0,
                movement.new_position.1,
                movement.pixel_type,
            ); // Then we insert that pixel into a new key
            self.last_updates
                .insert(movement.new_position, movement.pixel_type); // And also insert it into the updated hashmap
        }
        // Return an empty vector for now

        cross_chunk_movements
    }

    pub fn update_texture(&mut self) {
        let mut image = Image::gen_image_color(
            CHUNK_SIZE.0 as u16,
            CHUNK_SIZE.1 as u16,
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
        );

        for y in 0..CHUNK_SIZE.1 {
            for x in 0..CHUNK_SIZE.0 {
                if let Some(pixel_type) = self.get(x as i32, y as i32) {
                    let color = pixel_type.to_color();
                    image.set_pixel(x as u32, y as u32, color);
                }
            }
        }

        self.texture.update(&image);
    }

    pub fn draw(&self, chunk_key_x: i32, chunk_key_y: i32) {
        let chunk_x = chunk_key_x * CHUNK_SIZE.0 as i32;
        let chunk_y = chunk_key_y * CHUNK_SIZE.1 as i32;

        draw_texture_ex(
            &self.texture,
            chunk_x as f32,
            chunk_y as f32,
            WHITE,
            DrawTextureParams {
                ..Default::default()
            },
        );

        /*
        // Here we loop over the pixel grid to draw all the pixels
        for y in 0..CHUNK_SIZE.1 {
            for x in 0..CHUNK_SIZE.0 {
                let index = Chunk::index(x as i32, y as i32);
                if let Some(pixel_type) = self.chunk.get(index) {
                    draw_pixel(*pixel_type, chunk_x + x as i32, chunk_y + y as i32);
                }
            }
        }
        */
    }

    pub fn query(&self, x: i32, y: i32) -> GridQuery {
        // First check if the position is out of bounds
        if y >= self.height() || y < 0 {
            return GridQuery::OutOfBounds;
        }
        if x >= self.width() || x < 0 {
            return GridQuery::OutOfBounds;
        }
        // If it is not out of bounds, check if there is a pixel in the position
        if let Some(pixel_type) = self.get(x, y) {
            if *pixel_type == PixelType::Air {
                GridQuery::None
            } else {
                GridQuery::Hit(*pixel_type)
            }
        } else {
            GridQuery::None
        }
    }
    pub fn index(x: i32, y: i32) -> usize {
        (y * CHUNK_SIZE.0 as i32 + x) as usize
    }
    pub fn get(&self, x: i32, y: i32) -> Option<&PixelType> {
        let index = Chunk::index(x, y);
        self.chunk.get(index)
    }
    pub fn set(&mut self, x: i32, y: i32, pixel: PixelType) {
        let index = Chunk::index(x, y);
        self.chunk[index] = pixel;
    }
    pub fn remove(&mut self, x: i32, y: i32) -> PixelType {
        let index = Chunk::index(x, y);
        let old = self.chunk[index];
        self.chunk[index] = PixelType::Air;
        old
    }
    pub fn clear(&mut self) {
        self.chunk.clear();
        for _ in 0..CHUNK_SIZE.0 {
            for _ in 0..CHUNK_SIZE.1 {
                self.chunk.push(PixelType::Air);
            }
        }
    }
    pub fn width(&self) -> i32 {
        self.width
    }
    pub fn height(&self) -> i32 {
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
            GridQuery::OutOfBounds => true,
            GridQuery::Hit(_) => false,
            GridQuery::None => true,
        }
    }
}

pub struct GridMovement {
    pub old_position: (i32, i32),
    pub new_position: (i32, i32),
    pub old_chunk: Option<(i32, i32)>,
    pub new_chunk: Option<(i32, i32)>,
    pub pixel_type: PixelType,
}
impl GridMovement {
    pub fn new(old_position: (i32, i32), new_position: (i32, i32), pixel_type: PixelType) -> Self {
        Self {
            old_position,
            new_position,
            old_chunk: None,
            new_chunk: None,
            pixel_type,
        }
    }

    pub fn out_of_bounds(&self) -> bool {
        if self.new_position.0 as usize >= CHUNK_SIZE.0 || self.new_position.0 < 0 {
            return true;
        }
        if self.new_position.1 as usize >= CHUNK_SIZE.1 || self.new_position.1 < 0 {
            return true;
        }
        return false;
    }

    pub fn set_chunk_keys(&mut self, current_chunk_key: (i32, i32)) {
        self.old_chunk = Some(current_chunk_key);
        if !self.out_of_bounds() {
            self.new_chunk = self.old_chunk;
            return;
        }

        let mut new_chunk = current_chunk_key;

        let mut x = self.new_position.0;
        let mut y = self.new_position.1;
        // X Axis
        if x >= CHUNK_SIZE.0 as i32 {
            new_chunk.0 += 1;
            x -= CHUNK_SIZE.0 as i32;
        }
        if x < 0 {
            new_chunk.0 -= 1;
            x += CHUNK_SIZE.0 as i32;
        }

        // Y axis
        if y >= CHUNK_SIZE.1 as i32 {
            new_chunk.1 += 1;
            y -= CHUNK_SIZE.1 as i32;
            println!("TEST");
        }
        if y < 0 {
            new_chunk.1 -= 1;
            y += CHUNK_SIZE.1 as i32;
        }
        self.new_position = (x, y);
        self.new_chunk = Some(new_chunk);
    }
}
