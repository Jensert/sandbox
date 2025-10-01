use macroquad::math::Vec2;

use crate::{pixel::PixelType, pixel_grid::PixelGrid};

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
            pixel_type: PixelType::Sand,
            brush_type: BrushType::Circle,
            brush_size: 1.0,
        }
    }

    pub fn draw(&self, world_position: Vec2, pixel_grid: &mut PixelGrid) {
        match self.brush_type {
            BrushType::Pixel => self.draw_pixel(world_position, pixel_grid),
            BrushType::Circle => self.draw_circle(self.brush_size, world_position, pixel_grid),
        }
        pixel_grid.grid_mut().insert(
            (world_position.x as u32, world_position.y as u32),
            self.pixel_type,
        );
    }

    pub fn draw_pixel(&self, world_position: Vec2, pixel_grid: &mut PixelGrid) {
        pixel_grid.grid_mut().insert(
            (world_position.x as u32, world_position.y as u32),
            self.pixel_type,
        );
    }

    pub fn draw_circle(&self, radius: f32, center: Vec2, pixel_grid: &mut PixelGrid) {
        // Naive circle drawing
        for y in 0..pixel_grid.height() {
            let dy = y as f32 - center.y;
            for x in 0..pixel_grid.width() {
                let dx = x as f32 - center.x;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist <= radius - 1.0 as f32 {
                    pixel_grid
                        .grid_mut()
                        .insert((x as u32, y as u32), self.pixel_type);
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
