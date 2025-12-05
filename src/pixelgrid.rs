use crate::{CHUNK_SIZE, pixel::Pixel, pixeltype::PixelType};
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
    pub fn _new(chunk_key: (i32, i32), chunk_coordinate: (i32, i32)) -> Self {
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
// Do not implement copy or clone
pub struct ChunkGrid {
    grid: HashMap<(i32, i32), Chunk>,
}

impl ChunkGrid {
    pub fn new(rng: &RandGenerator) -> Self {
        let mut grid = HashMap::new();
        grid.insert((0, 0), Chunk::new(CHUNK_SIZE, rng, (0, 0)));
        grid.insert((0, 1), Chunk::new(CHUNK_SIZE, rng, (0, 1)));
        grid.insert((1, 0), Chunk::new(CHUNK_SIZE, rng, (1, 0)));
        grid.insert((1, 1), Chunk::new(CHUNK_SIZE, rng, (1, 1)));
        Self { grid }
    }

    pub fn update(&mut self, rng: &RandGenerator) {
        // Updating should be multiple stages:
        // First: apply all in-chunk movements
        // Second: get all cross-chunk movements for each chunk
        // Third: apply all cross-chunk movements
        let mut cross_chunk_movements: Vec<Vec<GridMovement>> = vec![];
        for ((_x, _y), chunk) in self.grid.iter_mut() {
            cross_chunk_movements.push(chunk.update(rng)); // Update all in-chunk movements and return all crosschunk movements
        }

        // Apply all cross chunk movements
        for chunk in cross_chunk_movements {
            for movement in chunk {
                match movement.new_chunk {
                    None => {
                        println!("chunk key not set! skipping movement");
                    }
                    Some(chunk_key) => {
                        // We have to remove the pixel from the old position here
                        if self.is_free(&movement) {
                            // Get the old chunk where the pixel has to be removed
                            let old_chunk = self.grid.get_mut(&movement.old_chunk.unwrap());
                            // Check if the chunk exists
                            if let Some(chunk) = old_chunk {
                                // Get the pixel from the old chunk
                                let pixel =
                                    chunk.remove(movement.old_position.0, movement.old_position.1);
                                // Get the new chunk where the pixel should move
                                let chunk = self.grid.get_mut(&chunk_key);
                                match chunk {
                                    // Check if the new chunk exists
                                    Some(chunk) => {
                                        // Check if new chunk position is free
                                        if chunk
                                            .query(movement.new_position.0, movement.new_position.1)
                                            .is_free()
                                        {
                                            // Set the pixel in the new chunk
                                            chunk.set(
                                                movement.new_position.0,
                                                movement.new_position.1,
                                                pixel,
                                            );
                                        }
                                    }
                                    None => unreachable!(), // Do nothing because chunk does not exist
                                                            // If this None branch is reached, then the pixel will simply be removed
                                                            // from the simulation. Because the pixel is removed from the chunk from the old
                                                            // check, but is never readded in the new position. Currently this is fine, but
                                                            // wil probnably spawn some unwanted bugs later on
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn grid_mut(&mut self) -> &mut HashMap<(i32, i32), Chunk> {
        &mut self.grid
    }

    pub fn _update_all_textures(&mut self) {
        for ((_, _), chunk) in self.grid.iter_mut() {
            chunk.update_textures();
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

    pub fn draw(&self, shader_strength: f32) {
        for ((chunk_key_x, chunk_key_y), chunk) in self.grid.iter() {
            chunk.draw(*chunk_key_x, *chunk_key_y, shader_strength);
        }
    }

    pub fn draw_borders(&self, render_ratio: (f32, f32)) {
        for ((chunk_key_x, chunk_key_y), chunk) in self.grid.iter() {
            chunk.draw_border(*chunk_key_x, *chunk_key_y, render_ratio);
        }
    }

    pub fn set_pixel(&mut self, world_position: Vec2, pixel: Pixel) {
        let chunk_position = ChunkPosition::from_world_position(world_position);
        self.grid.get_mut(&chunk_position.chunk_key).unwrap().set(
            chunk_position.chunk_coordinate.0,
            chunk_position.chunk_coordinate.1,
            pixel,
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
                if let Some(chunk) = self.grid.get(&chunk_key) {
                    if chunk
                        .query(grid_movement.new_position.0, grid_movement.new_position.1)
                        .is_free()
                    {
                        true
                    } else {
                        false
                    }
                } else {
                    // If chunk does not exist, do not move the pixel
                    false
                }
            }
        }
    }

    /// Does the same check as is_free(). It checks the chunk coordinate
    /// if it is free. But this returns the chunk if it is free.
    /// Can be used to directly insert into the chunk that is checked.
    pub fn _get_chunk_if_free(&self, grid_movement: &GridMovement) -> Option<&Chunk> {
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

// Do not implement copy or clone
pub struct Chunk {
    width: i32,
    height: i32,
    key: (i32, i32),
    chunk: Vec<Pixel>,
    last_updates: HashMap<(i32, i32), Pixel>,

    texture_main: Texture2D,
    shader_texture: Texture2D,

    updated_last_frame: bool,
}
impl Chunk {
    pub fn new(size: (usize, usize), _rng: &RandGenerator, key: (i32, i32)) -> Self {
        let chunk = vec![Pixel::empty(); CHUNK_SIZE.0 as usize * CHUNK_SIZE.1 as usize];
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

        let main_texture = Texture2D::from_image(&image);
        let shader_texture = main_texture.clone();
        main_texture.set_filter(FilterMode::Nearest);
        shader_texture.set_filter(FilterMode::Linear);

        Self {
            width: size.0 as i32,
            height: size.1 as i32,
            key,
            chunk,
            last_updates,

            texture_main: main_texture,
            shader_texture,

            updated_last_frame: false,
        }
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

        // If no changes happened this frame, dont do anything
        if changes.is_empty() {
            if self.updated_last_frame {
                self.update_textures();
            }
            self.updated_last_frame = false;
            return vec![]; // return empty vector
        } else {
            // Apply chanches if there are any
            self.updated_last_frame = true;
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
                let pixel = self.remove(movement.old_position.0, movement.old_position.1); // First we remove the pixel from the key at the old position
                self.set(movement.new_position.0, movement.new_position.1, pixel); // Then we insert that pixel into a new key
                self.last_updates.insert(movement.new_position, pixel); // And also insert it into the updated hashmap
            }

            self.update_textures(); // Update the texture to apply the changes visually
            // Return an empty vector for now

            cross_chunk_movements
        }
    }

    /// Update the chunks texture
    /// Currently this is called every frame.
    /// Should probably only be called if there is a change in the chunk,
    /// but this is fine for now
    pub fn update_textures(&mut self) {
        self.update_main_teture();
        self.update_shader_teture();
    }

    pub fn update_main_teture(&mut self) {
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
                if let Some(pixel) = self.get(x as i32, y as i32) {
                    let color = pixel.color();
                    image.set_pixel(x as u32, y as u32, color);
                }
            }
        }

        self.texture_main.update(&image);
    }

    pub fn update_shader_teture(&mut self) {
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
                if let Some(pixel) = self.get(x as i32, y as i32) {
                    let color = pixel.color();
                    image.set_pixel(x as u32, y as u32, color);
                }
            }
        }

        self.shader_texture.update(&image);
    }

    /// Draw the chunk's texture to the screen in the appropriate coordinates
    /// The chunk key are transformed to screen coordinates
    pub fn draw(&self, chunk_key_x: i32, chunk_key_y: i32, shader_strength: f32) {
        let chunk_x = chunk_key_x * CHUNK_SIZE.0 as i32;
        let chunk_y = chunk_key_y * CHUNK_SIZE.1 as i32;

        draw_texture_ex(
            &self.texture_main,
            chunk_x as f32,
            chunk_y as f32,
            WHITE,
            DrawTextureParams {
                ..Default::default()
            },
        );

        draw_texture_ex(
            &self.shader_texture,
            chunk_x as f32,
            chunk_y as f32,
            Color::new(1.0, 1.0, 1.0, shader_strength),
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

    pub fn draw_border(&self, chunk_key_x: i32, chunk_key_y: i32, render_ratio: (f32, f32)) {
        let x_adjust = CHUNK_SIZE.0 as f32 * render_ratio.0;
        let y_adjust = CHUNK_SIZE.1 as f32 * render_ratio.1;

        let x = chunk_key_x as f32 * x_adjust;
        let y = chunk_key_y as f32 * y_adjust;

        // Top border
        draw_line(x, y, x + x_adjust, y, 1.0, WHITE);
        // Bottom border
        draw_line(x, y + y_adjust, x + x_adjust, y + y_adjust, 1.0, WHITE);
        // Left border
        draw_line(x, y, x, y + y_adjust, 1.0, WHITE);
        // Right border
        draw_line(x + x_adjust, y, x + x_adjust, y + y_adjust, 1.0, WHITE);
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
        if let Some(pixel) = self.get(x, y) {
            if pixel.pixel_type() == PixelType::Air {
                GridQuery::None
            } else {
                GridQuery::Hit(*pixel)
            }
        } else {
            GridQuery::None
        }
    }
    pub fn index(x: i32, y: i32) -> usize {
        (y * CHUNK_SIZE.0 as i32 + x) as usize
    }
    pub fn get(&self, x: i32, y: i32) -> Option<&Pixel> {
        let index = Chunk::index(x, y);
        self.chunk.get(index)
    }
    pub fn set(&mut self, x: i32, y: i32, pixel: Pixel) {
        let index = Chunk::index(x, y);
        self.chunk[index] = pixel;
        self.updated_last_frame = true;
    }
    pub fn remove(&mut self, x: i32, y: i32) -> Pixel {
        let index = Chunk::index(x, y);
        let old = self.chunk[index];
        self.chunk[index] = Pixel::empty();
        old
    }
    pub fn clear(&mut self) {
        self.chunk.clear();
        for _ in 0..CHUNK_SIZE.0 {
            for _ in 0..CHUNK_SIZE.1 {
                self.chunk.push(Pixel::empty());
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

pub enum GridQuery {
    OutOfBounds,
    Hit(Pixel),
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
}
impl GridMovement {
    pub fn new(old_position: (i32, i32), new_position: (i32, i32)) -> Self {
        Self {
            old_position,
            new_position,
            old_chunk: None,
            new_chunk: None,
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
        }
        if y < 0 {
            new_chunk.1 -= 1;
            y += CHUNK_SIZE.1 as i32;
        }
        self.new_position = (x, y);
        self.new_chunk = Some(new_chunk);
    }
}
