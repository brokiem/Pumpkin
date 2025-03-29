use noise::Vector2;
use pumpkin_util::{
    math::{position::BlockPos, vector3::Vector3},
    random::RandomGenerator,
};
use serde::Deserialize;

use crate::generation::height_provider::HeightProvider;

#[derive(Deserialize)]
pub struct PlacedFeatureEntry {
    feature: String,
    placement: Vec<PlacementModifier>,
}

#[derive(Deserialize)]
pub enum PlacementModifier {
    BlockPredicateFilter,
    RarityFilter(RarityFilterPlacementModifier),
    SurfaceRelativeThresholdFilter,
    SurfaceWaterDepthFilter,
    Biome,
    Count,
    NoiseBasedCount,
    NoiseThresholdCount,
    CountOnEveryLayer,
    EnvironmentScan,
    Heightmap,
    HeightRange,
    InSquare,
    RandomOffset,
    FixedPlacement,
}

#[derive(Deserialize)]
pub struct RarityFilterPlacementModifier {
    chance: u32,
}

impl RarityFilterPlacementModifier {
    pub fn should_place(&self, random: &mut RandomGenerator) -> bool {
        random.next_f32() < 1.0 / self.chance as f32
    }
}

pub struct SquarePlacementModifier;

impl SquarePlacementModifier {
    pub fn get_positions(random: &mut RandomGenerator, pos: BlockPos) -> BlockPos {
        let x = random.next_bounded_i32(16) + pos.0.x;
        let z = random.next_bounded_i32(16) + pos.0.z;
        BlockPos(Vector3::new(x, pos.0.y, z))
    }
}

pub struct CountPlacementModifier {
    count: u8,
}

pub struct HeightRangePlacementModifier {
    height: HeightProvider,
}

impl HeightRangePlacementModifier {
    pub fn get_positions(
        &self,
        min_y: i8,
        height: u16,
        random: &mut RandomGenerator,
        pos: BlockPos,
    ) -> BlockPos {
        let mut pos = pos.clone();
        pos.0.y = self.height.get(random, min_y, height);
        pos
    }
}

pub struct HeightmapPlacementModifier {
    heightmap: String,
}

impl HeightmapPlacementModifier {
    pub fn get_positions(
        &self,
        min_y: i8,
        height: u16,
        random: &mut RandomGenerator,
        pos: BlockPos,
    ) -> BlockPos {
        let x = pos.0.x;
        let z = pos.0.z;
    }
}
