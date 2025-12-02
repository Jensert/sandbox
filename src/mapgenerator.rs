use crate::{
    pixelgrid::{self, Chunk, ChunkGrid},
    pixeltype::PixelType,
};

pub struct MapGenerator {
    layer_rules: Vec<MapLayerRule>,
}
impl MapGenerator {
    pub fn new(layer_rules: Vec<MapLayerRule>) -> Self {
        Self { layer_rules }
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
