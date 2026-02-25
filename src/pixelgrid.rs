use crate::{
    CHUNK_SIZE, RENDER_SIZE, noise,
    pixel::Pixel,
    pixeltype::{PixelMatter, PixelState, PixelType},
};
use macroquad::{prelude::*, rand::RandGenerator};
use std::collections::{HashMap, HashSet};

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
    image: Image,
    texture: Texture2D,
}

impl ChunkGrid {
    pub fn new(render_ratio: (f32, f32), seed: u64) -> Self {
        let mut grid = HashMap::new();
        grid.insert((0, 0), Chunk::new(CHUNK_SIZE, seed, (0, 0)));
        grid.insert((0, 1), Chunk::new(CHUNK_SIZE, seed, (0, 1)));
        grid.insert((1, 0), Chunk::new(CHUNK_SIZE, seed, (1, 0)));
        grid.insert((1, 1), Chunk::new(CHUNK_SIZE, seed, (1, 1)));

        let image = Image::gen_image_color(
            (RENDER_SIZE.0 as f32 * render_ratio.0) as u16,
            (RENDER_SIZE.1 as f32 * render_ratio.1) as u16,
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
        );
        let texture = Texture2D::from_image(&image);
        Self {
            grid,
            image,
            texture,
        }
    }

    pub fn update(&mut self, rng: &RandGenerator) {
        // Updating should be multiple stages:
        // First: get all movements for each chunk
        // Second: apply all movements
        let mut chunk_movements: Vec<Vec<GridMovement>> = vec![];
        for ((_x, _y), chunk) in self.grid.iter_mut() {
            chunk_movements.push(chunk.get_chunk_movements(rng)); // Update all in-chunk movements and return all crosschunk movements
        }

        // Apply all cross chunk movements
        for chunk in chunk_movements {
            for movement in chunk {
                match movement.new_chunk {
                    None => {
                        panic!(
                            "chunk key not set! check Chunk.update() function to make sure chunk key is always set"
                        );
                    }
                    Some(_) => {
                        // We have to remove the pixel from the old position here
                        if self.is_free(&movement) {
                            let old_world_position = {
                                (
                                    movement.old_position.0
                                        + (movement.old_chunk.unwrap().0 * CHUNK_SIZE.0 as i32),
                                    movement.old_position.1
                                        + (movement.old_chunk.unwrap().1 * CHUNK_SIZE.1 as i32),
                                )
                            };
                            let new_world_position = {
                                (
                                    movement.new_position.0
                                        + (movement.new_chunk.unwrap().0 * CHUNK_SIZE.0 as i32),
                                    movement.new_position.1
                                        + (movement.new_chunk.unwrap().1 * CHUNK_SIZE.1 as i32),
                                )
                            };
                            if let Some(pixel) = self.erase_pixel(vec2(
                                old_world_position.0 as f32,
                                old_world_position.1 as f32,
                            )) {
                                self.set_pixel(
                                    vec2(new_world_position.0 as f32, new_world_position.1 as f32),
                                    pixel,
                                );
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

    pub fn clear(&mut self) {
        for ((_x, _y), chunk) in self.grid.iter_mut() {
            chunk.clear();
        }
    }

    pub fn regenerate_chunks(&mut self, seed: u64) {
        let mut keys = vec![];
        for (x, y) in self.grid.keys() {
            keys.push((*x, *y));
        }
        for (x, y) in keys {
            self.grid_mut()
                .insert((x, y), Chunk::new(CHUNK_SIZE, seed, (x, y)));
        }
    }

    pub fn generate_chunk_if_not_exists(&mut self, seed: u64, chunk_key: (i32, i32)) {
        match self.grid.get(&chunk_key) {
            None => {
                println!("generating chunk at {chunk_key:?}");
                self.grid
                    .insert(chunk_key, Chunk::new(CHUNK_SIZE, seed, chunk_key));
            }
            Some(_) => {}
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
            chunk.draw_texture(*chunk_key_x, *chunk_key_y);
        }
    }

    pub fn draw_borders(&self, render_ratio: (f32, f32)) {
        for ((chunk_key_x, chunk_key_y), chunk) in self.grid.iter() {
            chunk.draw_border(*chunk_key_x, *chunk_key_y, render_ratio);
        }
    }

    pub fn draw_stability_to_texture(&mut self, render_ratio: (f32, f32)) {
        for ((chunk_key_x, chunk_key_y), chunk) in self.grid.iter() {
            let chunk_world_x = *chunk_key_x as f32 * CHUNK_SIZE.0 as f32;
            let chunk_world_y = *chunk_key_y as f32 * CHUNK_SIZE.1 as f32;

            for y in 0..CHUNK_SIZE.1 {
                for x in 0..CHUNK_SIZE.0 {
                    if let Some(pixel) = chunk.get(x as i32, y as i32) {
                        if pixel.pixel_type() == PixelType::Air {
                            continue;
                        }
                        let color =
                            Color::new(1.0 - pixel.stability(), pixel.stability(), 0.0, 1.0);
                        let screen_x = (chunk_world_x + x as f32) * render_ratio.0;
                        let screen_y = (chunk_world_y + y as f32) * render_ratio.1;
                        self.image
                            .set_pixel(screen_x as u32, screen_y as u32, color);
                    }
                }
            }
        }
        self.texture = Texture2D::from_image(&self.image);
    }

    pub fn draw_texture(&self) {
        draw_texture_ex(
            &self.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: None,
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
    }

    /// Set pixel to a position based on world_position
    /// This function already does boundary checks to make sure
    /// that coordinates out of bounds are skipped to prevent panics
    pub fn set_pixel(&mut self, world_position: Vec2, pixel: Pixel) {
        let chunk_position = ChunkPosition::from_world_position(world_position);
        if let Some(chunk) = self.grid.get_mut(&chunk_position.chunk_key) {
            chunk.set(
                chunk_position.chunk_coordinate.0,
                chunk_position.chunk_coordinate.1,
                pixel,
            )
        }

        self.propagate_stability_change(world_position);
    }

    /// Remove pixel based on world_position
    /// This function already does boundary checks to make sure
    /// that coordinates out of bounds are skipped to prevent panics
    pub fn erase_pixel(&mut self, world_position: Vec2) -> Option<Pixel> {
        let chunk_position = ChunkPosition::from_world_position(world_position);
        let erased_pixel = {
            let chunk = self.grid.get_mut(&chunk_position.chunk_key)?;
            Some(chunk.remove(
                chunk_position.chunk_coordinate.0,
                chunk_position.chunk_coordinate.1,
            ))
        };
        self.propagate_stability_change(world_position);
        erased_pixel
    }

    /// Function that is called whenever a Pixel is set() or remove() from a Chunk
    /// This updates the stability for that pixel that is set or removed,
    /// and also updates the stability for the left, right and top neighbouring pixels
    pub fn propagate_stability_change(&mut self, world_position: Vec2) {
        let x = world_position.x as i32;
        let y = world_position.y as i32;
        // Use a queue to process stability changes
        let mut to_check = vec![(x, y)];

        let mut checked = HashSet::new();

        while let Some((x, y)) = to_check.pop() {
            if !checked.insert((x, y)) {
                continue; // already processed
            }

            // If out of bounds then skip
            if x < 0 || y < 0 || x >= RENDER_SIZE.0 as i32 || y >= RENDER_SIZE.1 as i32 - 1 {
                continue;
            }
            // Phase 1: calclulate stability and return the stability and state in a tuple
            let (new_stability, new_state) = { self.calculate_pixel_stability(x, y) };

            // Phase 2: write stabilities and states to the pixels
            if let Some(pixel) = self.get_pixel_mut(x, y) {
                let old_stability = pixel.stability();
                let old_state = pixel.state();

                if old_stability != new_stability || old_state != new_state {
                    pixel.set_stability(new_stability);
                    pixel.set_state(new_state);
                    to_check.push((x - 1, y)); // Enqueue left
                    to_check.push((x + 1, y)); // Enqueue right
                    to_check.push((x, y - 1)); // Enqueue above
                }
            }
        }
    }

    /// Get the pixel stability of the given world coordinate
    /// This returns a tuple with a float and a PixelState
    /// if the world coordinate is out of bounds
    /// The tuple will contain (0.0, PixelState::Stable)
    pub fn calculate_pixel_stability(&self, world_x: i32, world_y: i32) -> (f32, PixelState) {
        if let Some(pixel) = self.get_pixel(world_x, world_y) {
            // If pixel is not solid (like Water) stability = 0.0
            if pixel.matter() != PixelMatter::Solid {
                return (0.0, PixelState::Falling);
            }

            // If floor of map is reached or Pixel has no decay, then always return stable
            if world_y >= RENDER_SIZE.1 as i32 - 1 || pixel.stability_decay() == 0.0 {
                return (1.0, PixelState::Stable);
            }

            // If pixel below is solid and has stability then return that pixels stability + (1 - stability decay)
            if let Some(p_below) = self.query_world(world_x, world_y + 1).is_occupied() {
                if p_below.matter() == PixelMatter::Solid && p_below.stability() > 0.0 {
                    return (
                        clamp(
                            p_below.stability() + (1.0 - pixel.stability_decay()),
                            0.0,
                            1.0,
                        ),
                        PixelState::Stable,
                    );
                }
            }

            // if the pixel is stable, inherit stability from left and right
            if pixel.state() == PixelState::Stable {
                // get left and right neighbouring stabilities
                let pixel_left = self
                    .query_world(world_x - 1, world_y)
                    .is_occupied()
                    .map(|p| p.stability())
                    .unwrap_or(0.0);
                let pixel_right = self
                    .query_world(world_x + 1, world_y)
                    .is_occupied()
                    .map(|p| p.stability())
                    .unwrap_or(0.0);

                // Take maximum neighbour stability
                let max_neighbour_stability = pixel_left.max(pixel_right);
                // Subtract this pixels decay rate
                let stability = clamp(max_neighbour_stability - pixel.stability_decay(), 0.0, 1.0);

                let state = if stability > 0.0 {
                    PixelState::Stable
                } else {
                    PixelState::Falling
                };

                return (stability, state);
            }
            (pixel.stability(), pixel.state())
        } else {
            // Out of bounds returns stable pixel for now
            return (1.0, PixelState::Stable);
        }
    }

    /// full stability recalculation for debugging
    pub fn recalculate_all_stability(&mut self) {
        println!("Recalculating all stability");
        // Bottom to top (.rev()) to ensure correct propagation
        for y in (0..RENDER_SIZE.1 as i32).rev() {
            for x in 0..RENDER_SIZE.0 as i32 {
                let (stability, state) = self.calculate_pixel_stability(x, y);
                if let Some(pixel) = self.get_pixel_mut(x, y) {
                    pixel.set_stability(stability);
                    pixel.set_state(state);
                };
            }
        }
    }

    /// Query the ChunkGrid with world coordiantes
    /// Returns a GridQuery with the resulting *Pixel
    /// or an OutOfBounds / None result
    pub fn query_world(&self, world_x: i32, world_y: i32) -> GridQuery {
        let chunk_key_x = world_x.div_euclid(CHUNK_SIZE.0 as i32);
        let chunk_key_y = world_y.div_euclid(CHUNK_SIZE.1 as i32);
        let chunk_x = world_x.rem_euclid(CHUNK_SIZE.0 as i32);
        let chunk_y = world_y.rem_euclid(CHUNK_SIZE.1 as i32);
        if let Some(chunk) = self.grid.get(&(chunk_key_x, chunk_key_y)) {
            if let Some(pixel) = chunk.get(chunk_x, chunk_y) {
                GridQuery::Hit(*pixel)
            } else {
                GridQuery::None
            }
        } else {
            GridQuery::OutOfBounds
        }
    }

    pub fn _query_world_with_chunk_key(
        &self,
        chunk_key: (i32, i32),
        chunk_x: i32,
        chunk_y: i32,
    ) -> GridQuery {
        if let Some(chunk) = self.grid.get(&chunk_key) {
            if let Some(pixel) = chunk.get(chunk_x, chunk_y) {
                GridQuery::Hit(*pixel)
            } else {
                GridQuery::OutOfBounds
            }
        } else {
            GridQuery::OutOfBounds
        }
    }
    /// Get unmutable pixel reference from the ChunkGrid with world coordinates
    /// Returns an Option<&Pixel>
    pub fn get_pixel(&self, world_x: i32, world_y: i32) -> Option<&Pixel> {
        let chunk_key_x = world_x.div_euclid(CHUNK_SIZE.0 as i32);
        let chunk_key_y = world_y.div_euclid(CHUNK_SIZE.1 as i32);
        let chunk_x = world_x.rem_euclid(CHUNK_SIZE.0 as i32);
        let chunk_y = world_y.rem_euclid(CHUNK_SIZE.1 as i32);
        if let Some(chunk) = self.grid.get(&(chunk_key_x, chunk_key_y)) {
            if let Some(pixel) = chunk.get(chunk_x, chunk_y) {
                Some(pixel)
            } else {
                None
            }
        } else {
            None
        }
    }
    /// Get a mutable pixel reference from the ChunkGrid with world coordinates
    /// Returns an Option<&mut Pixel>
    pub fn get_pixel_mut(&mut self, world_x: i32, world_y: i32) -> Option<&mut Pixel> {
        let chunk_key_x = world_x.div_euclid(CHUNK_SIZE.0 as i32);
        let chunk_key_y = world_y.div_euclid(CHUNK_SIZE.1 as i32);
        let chunk_x = world_x.rem_euclid(CHUNK_SIZE.0 as i32);
        let chunk_y = world_y.rem_euclid(CHUNK_SIZE.1 as i32);
        if let Some(chunk) = self.grid.get_mut(&(chunk_key_x, chunk_key_y)) {
            if let Some(pixel) = chunk.get_mut(chunk_x, chunk_y) {
                Some(pixel)
            } else {
                None
            }
        } else {
            None
        }
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

    image: Image,
    texture: Texture2D,

    updated_last_frame: bool,
}
impl Chunk {
    pub fn _empty(size: (usize, usize), _rng: &RandGenerator, key: (i32, i32)) -> Self {
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

        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Nearest);

        Self {
            width: size.0 as i32,
            height: size.1 as i32,
            key,
            chunk,
            last_updates,

            image,
            texture,

            updated_last_frame: true,
        }
    }

    pub fn new(size: (usize, usize), seed: u64, chunk_key: (i32, i32)) -> Self {
        let mut chunk = vec![];
        let last_updates = HashMap::new();

        let chunk_size = (CHUNK_SIZE.0 as i32, CHUNK_SIZE.1 as i32);

        for y in 0..chunk_size.1 {
            for x in 0..chunk_size.0 {
                let world_x = chunk_key.0 * chunk_size.0 + x;
                let world_y = chunk_key.1 * chunk_size.1 + y;

                let hash =
                    noise::noise2d(world_x as f32 * 0.05, world_y as f32 * 0.05, seed as i32);
                let pixel_type = if hash < 0.01 {
                    PixelType::HardStone
                } else if hash < 0.18 {
                    PixelType::Air
                } else if hash < 0.2 {
                    PixelType::HardStone
                } else {
                    PixelType::Stone
                };

                chunk.push(Pixel::from_pixel_type(pixel_type));
            }
        }

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

        println!("Chunk generated");

        Self {
            width: size.0 as i32,
            height: size.1 as i32,
            key: chunk_key,
            chunk,
            last_updates,

            image,
            texture,

            updated_last_frame: true,
        }
    }

    /// The update function returns a gridmovements. The return type is used
    /// by the parent struct ChunkGrid to handle all chunk movements.
    pub fn get_chunk_movements(&mut self, rng: &RandGenerator) -> Vec<GridMovement> {
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
                if let Some(pixel) = self.get(x as i32, y as i32) {
                    if let Some(mut movement) = pixel.update(self, x as i32, y as i32, rng) {
                        movement.set_chunk_keys(self.key);
                        changes.push(movement);
                    }
                }
            }
        }
        if self.updated_last_frame || !changes.is_empty() {
            self.update_textures();
        }
        return changes;
    }

    /// Update the chunks texture
    /// Currently this is called every frame.
    /// Should probably only be called if there is a change in the chunk,
    /// but this is fine for now
    pub fn update_textures(&mut self) {
        self.update_texture();
    }

    pub fn update_texture(&mut self) {
        for y in 0..CHUNK_SIZE.1 {
            for x in 0..CHUNK_SIZE.0 {
                if let Some(pixel) = self.get(x as i32, y as i32) {
                    let color = pixel.color();
                    self.image.set_pixel(x as u32, y as u32, color);
                }
            }
        }

        self.texture.update(&self.image);
    }

    /// Draw the chunk's texture to the screen in the appropriate coordinates
    /// The chunk key are transformed to screen coordinates
    pub fn draw_texture(&self, chunk_key_x: i32, chunk_key_y: i32) {
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

    pub fn _draw_stability(&self, chunk_key_x: i32, chunk_key_y: i32, render_ratio: (f32, f32)) {
        let chunk_world_x = chunk_key_x as f32 * CHUNK_SIZE.0 as f32;
        let chunk_world_y = chunk_key_y as f32 * CHUNK_SIZE.1 as f32;

        for y in 0..CHUNK_SIZE.1 {
            for x in 0..CHUNK_SIZE.0 {
                if let Some(pixel) = self.get(x as i32, y as i32) {
                    if pixel.pixel_type() != PixelType::Air {
                        let color =
                            Color::new(1.0 - pixel.stability(), pixel.stability(), 0.0, 1.0);
                        let screen_x = (chunk_world_x + x as f32) * render_ratio.0;
                        let screen_y = (chunk_world_y + y as f32) * render_ratio.1;
                        draw_rectangle(screen_x, screen_y, 1.0, 1.0, color);
                    }
                }
            }
        }
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
    /// Getter function for Chunk
    /// Returns an Option<&Pixel>
    pub fn get(&self, x: i32, y: i32) -> Option<&Pixel> {
        let index = Chunk::index(x, y);
        self.chunk.get(index)
    }
    /// Getter function for Chunk
    /// Returns an Option<&mut Pixel>
    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut Pixel> {
        let index = Chunk::index(x, y);
        self.chunk.get_mut(index)
    }
    /// Setter function for Chunk
    /// Sets the pixel to the position
    pub fn set(&mut self, x: i32, y: i32, pixel: Pixel) {
        let index = Chunk::index(x, y);
        self.chunk[index] = pixel;
        self.updated_last_frame = true;
    }
    pub fn remove(&mut self, x: i32, y: i32) -> Pixel {
        let index = Chunk::index(x, y);
        let old = self.chunk[index];
        self.chunk[index] = Pixel::empty();
        self.updated_last_frame = true;

        old
    }

    pub fn clear(&mut self) {
        self.chunk.clear();
        for _ in 0..CHUNK_SIZE.0 {
            for _ in 0..CHUNK_SIZE.1 {
                self.chunk.push(Pixel::empty());
                self.updated_last_frame = true;
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

    pub fn is_occupied(&self) -> Option<&Pixel> {
        match self {
            GridQuery::OutOfBounds => None,
            GridQuery::None => None,
            GridQuery::Hit(pixel) => Some(pixel),
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
