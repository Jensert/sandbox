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
        let mut map_generator = Self::empty();

        map_generator.push_layer(MapLayerRule::new(
            140,
            180,
            0, // lowest -> first
            PixelType::Lava,
            ProbabilityProfile::Constant(1.0),
        ));

        map_generator.push_layer(MapLayerRule::new(
            100,
            143,
            10, // above lava
            PixelType::HardStone,
            ProbabilityProfile::Custom(Box::new(|y, _, end| {
                if y < 140 {
                    1.0
                } else {
                    ProbabilityProfile::Linear {
                        start: 1.0,
                        end: 0.0,
                    }
                    .get(y, 140, end)
                }
            })),
        ));

        map_generator.push_layer(MapLayerRule::new(
            140,
            150,
            20,             // <---- your new AIR layer can go here
            PixelType::Air, // or PixelType::Empty
            ProbabilityProfile::Constant(1.0),
        ));

        map_generator.push_layer(MapLayerRule::new(
            50,
            106,
            30,
            PixelType::Stone,
            ProbabilityProfile::Custom(Box::new(|y, _, end| {
                if y < 50 {
                    1.0
                } else {
                    ProbabilityProfile::Linear {
                        start: 1.0,
                        end: 0.0,
                    }
                    .get(y, 100, end)
                }
            })),
        ));

        map_generator.push_layer(MapLayerRule::new(
            45,
            60,
            40,
            PixelType::Dirt,
            ProbabilityProfile::Custom(Box::new(|y, _, end| {
                if y < 50 {
                    1.0
                } else {
                    ProbabilityProfile::Linear {
                        start: 1.0,
                        end: 0.0,
                    }
                    .get(y, 50, end)
                }
            })),
        ));

        map_generator.push_layer(MapLayerRule::new(
            45,
            48,
            50,
            PixelType::Grass,
            ProbabilityProfile::Linear {
                start: 1.0,
                end: 0.0,
            },
        ));

        map_generator
    }
    /// Push a new layer into the map generator struct
    /// There is some safety built into this to make sure
    /// that the layers are pushed in correct order
    /// They must be pushed from lowest layer to top layer
    /// LavaLayer > DeepLayer > StoneLayer > DirtLayer > GrassLayer
    pub fn push_layer(&mut self, layer_rule: MapLayerRule) -> &mut Self {
        self.layer_rules.push(layer_rule);
        self.layer_rules.sort_by_key(|rule| rule.order());

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
    order: i32,
    pixel_type: PixelType,
    probability: ProbabilityProfile,
}
impl MapLayerRule {
    pub fn new(
        y_start: i32,
        y_end: i32,
        order: i32,
        pixel_type: PixelType,
        probability: ProbabilityProfile,
    ) -> Self {
        Self {
            y_start,
            y_end,
            order,
            pixel_type,
            probability,
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

    pub fn order(&self) -> i32 {
        self.order
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
