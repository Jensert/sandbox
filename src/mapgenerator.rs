use macroquad::{math::Vec2, rand::RandGenerator};

use crate::{
    RENDER_SIZE,
    pixel::Pixel,
    pixelgrid::{ChunkGrid, ChunkPosition},
    pixeltype::PixelType,
};

pub struct MapGenerator {
    layer_rules: Vec<MapLayerRule>,
}
impl MapGenerator {
    pub fn empty() -> Self {
        let layer_rules = vec![];
        Self { layer_rules }
    }
    pub fn default() -> Self {
        let lava_layer = MapLayerRule::from_default_layer_type(LayerType::LavaLayer);
        let deep_layer = MapLayerRule::from_default_layer_type(LayerType::DeepLayer);
        let stone_layer = MapLayerRule::from_default_layer_type(LayerType::StoneLayer);
        let dirt_layer = MapLayerRule::from_default_layer_type(LayerType::DirtLayer);
        let grass_layer = MapLayerRule::from_default_layer_type(LayerType::GrassLayer);

        let mut map_generator = Self::empty();

        map_generator
            .push_layer(lava_layer)
            .push_layer(deep_layer)
            .push_layer(stone_layer)
            .push_layer(dirt_layer)
            .push_layer(grass_layer);

        map_generator
    }

    /// Push a new layer into the map generator struct
    /// There is some safety built into this to make sure
    /// that the layers are pushed in correct order
    /// They must be pushed from lowest layer to top layer
    /// LavaLayer > DeepLayer > StoneLayer > DirtLayer > GrassLayer
    pub fn push_layer(&mut self, layer_rule: MapLayerRule) -> &mut Self {
        let top_layer = self.layer_rules.last();
        match top_layer {
            None => {
                if layer_rule.layer_type == LayerType::LavaLayer {
                    self.layer_rules.push(layer_rule);
                } else {
                    panic!("Map generator layers out of order");
                }
            }
            Some(top_layer) => {
                if layer_rule.layer_type <= top_layer.layer_type {
                    panic!("Map generator layers out of order");
                } else {
                    self.layer_rules.push(layer_rule);
                }
            }
        }

        self
    }

    pub fn generate_map(&self, chunk_grid: &mut ChunkGrid, rng: &RandGenerator) {
        for layer in &self.layer_rules {
            layer.generate_layer(chunk_grid, rng);
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
    pub fn from_default_layer_type(layer_type: LayerType) -> Self {
        match layer_type {
            LayerType::GrassLayer => {
                let y_start = 45;
                let y_end = 48;
                let pixel_type = PixelType::Grass;
                return Self {
                    y_start,
                    y_end,
                    pixel_type,
                    layer_type,
                    probability: ProbabilityProfile::Linear {
                        start: 1.0,
                        end: 0.0,
                    },
                };
            }
            LayerType::DirtLayer => {
                let y_start = 45;
                let y_end = 60;
                let pixel_type = PixelType::Dirt;
                return Self {
                    y_start,
                    y_end,
                    pixel_type,
                    layer_type,
                    probability: ProbabilityProfile::Custom(Box::new(|y, _, y_end| {
                        if y < 50 {
                            1.0
                        } else {
                            ProbabilityProfile::Linear {
                                start: 1.0,
                                end: 0.0,
                            }
                            .get(y, 50, y_end)
                        }
                    })),
                };
            }
            LayerType::StoneLayer => {
                let y_start = 45;
                let y_end = 100;
                let pixel_type = PixelType::Stone;
                return Self {
                    y_start,
                    y_end,
                    pixel_type,
                    layer_type,
                    probability: ProbabilityProfile::Constant(1.0),
                };
            }
            LayerType::DeepLayer => {
                let y_start = 100;
                let y_end = 140;
                let pixel_type = PixelType::HardStone;
                return Self {
                    y_start,
                    y_end,
                    pixel_type,
                    layer_type,
                    probability: ProbabilityProfile::Constant(1.0),
                };
            }
            LayerType::LavaLayer => {
                let y_start = 140;
                let y_end = 180;
                let pixel_type = PixelType::Lava;
                return Self {
                    y_start,
                    y_end,
                    pixel_type,
                    layer_type,
                    probability: ProbabilityProfile::Constant(1.0),
                };
            }
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
    _Parabola { a: f32, b: f32, c: f32 },
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
            ProbabilityProfile::_Parabola { a, b, c } => {
                let t = (y - y_start) as f32;
                (a * t * t + b * t + c).clamp(0.0, 1.0)
            }
            ProbabilityProfile::Custom(f) => (f)(y, y_start, y_end),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum LayerType {
    LavaLayer,
    DeepLayer,
    StoneLayer,
    DirtLayer,
    GrassLayer,
}
