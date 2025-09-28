use crate::pixel_grid::PixelGrid;
use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub enum PixelType {
    Sand,
    Water,
}

pub trait PixelBehaviour {
    fn update(&self, pixelgrid: &PixelGrid, x: u32, y: u32) -> Option<((u32, u32), (u32, u32))>;

    fn draw(&self, x: u32, y: u32);
}

pub fn draw_pixel(pixel_type: PixelType, x: u32, y: u32) {
    let x = x as f32;
    let y = y as f32;
    let w = 1.0;
    let h = 1.0;
    match pixel_type {
        PixelType::Sand => draw_rectangle(x, y, w, h, BROWN),
        PixelType::Water => draw_rectangle(x, y, w, h, BLUE),
    }
}

pub fn update_sand(
    pixelgrid: &PixelGrid,
    x: u32,
    y: u32,
) -> Option<((u32, u32), (u32, u32), PixelType)> {
    let mut res = None; // Res will be returned. contains current and new position and the pixel
    // Check if y position is bottom of the screen
    if y < pixelgrid.height() - 1 {
        // Check if there is no sand pixel below current position
        let mut new_pos = (x, y + 1); // If it isn't set new pos 1 pixel down
        let pixel_type = PixelType::Sand;
        if pixelgrid.get(new_pos).is_none() {
            res = Some(((x, y), new_pos, pixel_type)); // return the old pos, new pos and the pixel data
            return res;
        }

        // If there is a sand pixel below, check 1 down and right
        new_pos.0 += 1;
        if pixelgrid.get(new_pos).is_none() {
            res = Some(((x, y), new_pos, pixel_type)); // return the old pos, new pos and the pixel data
            return res;
        }

        // If there is a sand pixel below, check 1 down and left
        new_pos.0 -= 2;
        if pixelgrid.get(new_pos).is_none() {
            res = Some(((x, y), new_pos, pixel_type)); // return the old pos, new pos and the pixel data
            return res;
        }
    }
    return res;
}

pub fn update_water(
    pixelgrid: &PixelGrid,
    x: u32,
    y: u32,
) -> Option<((u32, u32), (u32, u32), PixelType)> {
    let mut res = None; // Res will be returned. contains current and new position and the pixel
    // Check if y position is bottom of the screen
    if y < pixelgrid.height() - 1 {
        // Check if there is no sand pixel below current position
        let mut new_pos = (x, y + 1); // If it isn't set new pos 1 pixel down
        let pixel_type = PixelType::Water;
        if pixelgrid.get(new_pos).is_none() {
            res = Some(((x, y), new_pos, pixel_type)); // return the old pos, new pos and the pixel data
            return res;
        }

        // If there is a sand pixel below, check 1 down and right
        new_pos.0 += 1;
        if pixelgrid.get(new_pos).is_none() {
            res = Some(((x, y), new_pos, pixel_type)); // return the old pos, new pos and the pixel data
            return res;
        }

        // If there is a sand pixel below, check 1 down and left
        new_pos.0 -= 2;
        if pixelgrid.get(new_pos).is_none() {
            res = Some(((x, y), new_pos, pixel_type)); // return the old pos, new pos and the pixel data
            return res;
        }
    }
    return res;
}
