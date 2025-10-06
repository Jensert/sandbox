use macroquad::math::{Vec2, vec2};

use crate::{
    CHUNK_SIZE,
    pixel::PixelType,
    pixel_grid::{Chunk, ChunkGrid},
};

#[derive(Clone, Copy)]
pub enum BrushType {
    Pixel,
    Circle,
}
impl BrushType {
    pub fn as_str(&self) -> &str {
        match *self {
            BrushType::Pixel => "Pixel",
            BrushType::Circle => "Circle",
        }
    }

    pub fn next(&mut self) {
        match *self {
            BrushType::Pixel => *self = BrushType::Circle,
            BrushType::Circle => *self = BrushType::Pixel,
        }
    }
    pub fn previous(&mut self) {
        match *self {
            BrushType::Circle => *self = BrushType::Pixel,
            BrushType::Pixel => *self = BrushType::Circle,
        }
    }
}
#[derive(Clone, Copy)]
pub struct Brush {
    pixel_type: PixelType,
    brush_type: BrushType,
    brush_size: f32,
}

impl Brush {
    pub fn new() -> Self {
        Self {
            pixel_type: PixelType::Dirt,
            brush_type: BrushType::Pixel,
            brush_size: 5.0,
        }
    }

    pub fn draw(&self, world_position: Vec2, chunk_grid: &mut ChunkGrid) {
        match self.brush_type {
            BrushType::Pixel => self.draw_pixel(world_position, chunk_grid),
            BrushType::Circle => self.draw_circle(self.brush_size, world_position, chunk_grid),
        }
    }

    pub fn draw_pixel(&self, world_position: Vec2, chunk_grid: &mut ChunkGrid) {
        let mut pos = world_position;
        for y in 0..self.brush_size as i32 {
            let dy = pos.y + y as f32;
            for x in 0..self.brush_size as i32 {
                let dx = pos.x + x as f32;
                chunk_grid.insert_pixel(vec2(dx, dy), self.pixel_type())
            }
        }
    }

    pub fn draw_circle(&self, radius: f32, center: Vec2, chunk_grid: &mut ChunkGrid) {
        // Naive circle drawing
        for y in 0..CHUNK_SIZE.1 {
            let dy = y as f32 - center.y;
            for x in 0..CHUNK_SIZE.0 {
                let dx = x as f32 - center.x;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist <= radius - 1.0 as f32 {
                    chunk_grid.insert_pixel(center, self.pixel_type());
                }
            }
        }
    }

    pub fn pixel_type(&self) -> PixelType {
        self.pixel_type
    }
    pub fn pixel_type_mut(&mut self) -> &mut PixelType {
        &mut self.pixel_type
    }
    pub fn brush_type(&self) -> BrushType {
        self.brush_type
    }
    pub fn brush_type_mut(&mut self) -> &mut BrushType {
        &mut self.brush_type
    }
    pub fn size(&self) -> f32 {
        self.brush_size
    }
    pub fn increase_size(&mut self, amount: f32) {
        self.brush_size += amount;
    }
    pub fn decrease_size(&mut self, amount: f32) {
        self.brush_size -= amount;
    }
}
