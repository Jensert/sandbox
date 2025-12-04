use macroquad::{math::Vec2, rand::RandGenerator};

use crate::{
    RENDER_SIZE,
    pixel::Pixel,
    pixelgrid::{self, Chunk, ChunkGrid, ChunkPosition},
    pixeltype::PixelType,
};

pub struct MapGenerator {
    layer_rules: Vec<MapLayerRule>,
}
impl MapGenerator {
    pub fn new(layer_rules: Vec<MapLayerRule>) -> Self {
        Self { layer_rules }
    }

    /// Push a new layer into the map generator struct
    /// There is some safety built into this to make sure
    /// that the layers are pushed in correct order
    /// They must be pushed from lowest layer to top layer
    /// LavaLayer > DeepLayer > StoneLayer > DirtLayer > GrassLayer
    pub fn push_layer(&mut self, layer_rule: MapLayerRule) {
        match layer_rule.layer_type {
            LayerType::LavaLayer => (),
            LayerType::DeepLayer => (),
            LayerType::StoneLayer => (),
            LayerType::DirtLayer => (),
            LayerType::GrassLayer => (),
        }
    }
}

pub struct MapLayerRule {
    y_start: i32,
    y_end: i32,
    pixel_type: PixelType,
    layer_type: LayerType,
    probability: ProbabilityProfile,
}
impl MapLayerRule {
    pub fn new(
        y_start: i32,
        y_end: i32,
        pixel_type: PixelType,
        layer_type: LayerType,
        probability: ProbabilityProfile,
    ) -> Self {
        Self {
            y_start,
            y_end,
            pixel_type,
            layer_type,
            probability,
        }
    }

    pub fn from_default_layer_type(layer_type: LayerType) -> Self {
        match layer_type {
            LayerType::GrassLayer => {
                let y_start = 45;
                let y_end = 72;
                let pixel_type = PixelType::Grass;
                let probability = return Self {
                    y_start,
                    y_end,
                    pixel_type,
                    layer_type,
                    probability: ProbabilityProfile::Parabola {
                        a: 0.0122,
                        b: -0.0019,
                        c: 0.0,
                    },
                };
            }
            LayerType::DirtLayer => unimplemented!(),
            LayerType::StoneLayer => unimplemented!(),
            LayerType::DeepLayer => unimplemented!(),
            LayerType::LavaLayer => unimplemented!(),
        }
    }

    pub fn generate_layer(&self, chunk_grid: &mut ChunkGrid, rng: &RandGenerator) {
        for y in self.y_start..self.y_end {
            // calculate probability here
            let chance = self.probability.get(y, self.y_start, self.y_end);
            for x in 0..RENDER_SIZE.0 {
                // If probability is reached then generate tile
                if rng.gen_range(0.0, 1.0) < chance {
                    let pos = Vec2::new(x as f32, y as f32);
                    let chunk_pos = ChunkPosition::from_world_position(pos);

                    if let Some(chunk) = chunk_grid.grid_mut().get_mut(&chunk_pos.chunk_key) {
                        chunk.set(
                            chunk_pos.chunk_coordinate.0,
                            chunk_pos.chunk_coordinate.1,
                            Pixel::from_pixel_type(self.pixel_type, &rng),
                        );
                    }
                }
            }
        }
    }
}

pub enum ProbabilityProfile {
    Constant(f32),
    Linear { start: f32, end: f32 },
    Parabola { a: f32, b: f32, c: f32 },
    Custom(Box<dyn Fn(i32, i32, i32) -> f32>), // fallback for weird cases
}

impl ProbabilityProfile {
    pub fn get(&self, y: i32, y_start: i32, y_end: i32) -> f32 {
        match self {
            ProbabilityProfile::Constant(p) => *p,
            ProbabilityProfile::Linear { start, end } => {
                let t = (y - y_start) as f32 / (y_end - y_start) as f32;
                start + t * (end - start)
            }
            ProbabilityProfile::Parabola { a, b, c } => {
                let t = (y - y_start) as f32;
                (a * t * t + b * t + c).clamp(0.0, 1.0)
            }
            ProbabilityProfile::Custom(f) => (f)(y, y_start, y_end),
        }
    }
}
pub enum LayerType {
    GrassLayer,
    DirtLayer,
    StoneLayer,
    DeepLayer,
    LavaLayer,
}
