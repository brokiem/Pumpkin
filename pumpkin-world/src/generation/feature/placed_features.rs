use pumpkin_util::{
    math::{int_provider::IntProvider, position::BlockPos, vector3::Vector3},
    random::RandomGenerator,
};
use serde::Deserialize;
use std::iter;

use crate::{ProtoChunk, generation::height_provider::HeightProvider};

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
    Biome(BiomePlacementModifier),
    Count(CountPlacementModifier),
    NoiseBasedCount,
    NoiseThresholdCount,
    CountOnEveryLayer,
    EnvironmentScan,
    Heightmap,
    HeightRange(HeightRangePlacementModifier),
    InSquare(SquarePlacementModifier),
    RandomOffset,
    FixedPlacement,
}

pub struct FeaturePlacementContext {
    placed_feature: String,
}

#[derive(Deserialize)]
pub struct RarityFilterPlacementModifier {
    chance: u32,
}

impl ConditionalPlacementModifier for RarityFilterPlacementModifier {
    fn should_place(
        &self,
        context: &FeaturePlacementContext,
        chunk: &ProtoChunk,
        random: &mut RandomGenerator,
        pos: BlockPos,
    ) -> bool {
        random.next_f32() < 1.0 / self.chance as f32
    }
}

#[derive(Deserialize)]
pub struct SquarePlacementModifier;

impl SquarePlacementModifier {
    pub fn get_positions(
        random: &mut RandomGenerator,
        pos: BlockPos,
    ) -> impl Iterator<Item = BlockPos> {
        let x = random.next_bounded_i32(16) + pos.0.x;
        let z = random.next_bounded_i32(16) + pos.0.z;
        iter::once(BlockPos(Vector3::new(x, pos.0.y, z)))
    }
}

#[derive(Deserialize)]
pub struct CountPlacementModifier {
    count: IntProvider,
}

impl CountPlacementModifierBase for CountPlacementModifier {
    fn get_count(&self) -> i32 {
        self.count.get()
    }
}

#[derive(Deserialize)]
pub struct BiomePlacementModifier;

impl ConditionalPlacementModifier for BiomePlacementModifier {
    fn should_place(
        &self,
        context: &FeaturePlacementContext,
        chunk: &ProtoChunk,
        random: &mut RandomGenerator,
        pos: BlockPos,
    ) -> bool {
        // we check if the current feature can be applied to the biome at the pos
        let features = chunk.get_biome(&pos.0).features.first().unwrap();
        let this_feature = &context.placed_feature;
        features.contains(&this_feature.as_str())
    }
}

#[derive(Deserialize)]
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

// impl HeightmapPlacementModifier {
//     pub fn get_positions(
//         &self,
//         min_y: i8,
//         height: u16,
//         random: &mut RandomGenerator,
//         pos: BlockPos,
//     ) -> BlockPos {
//         let x = pos.0.x;
//         let z = pos.0.z;
//     }
// }
//

pub trait CountPlacementModifierBase {
    fn get_positions(
        &self,
        _random: &mut RandomGenerator,
        pos: BlockPos,
    ) -> impl Iterator<Item = BlockPos> {
        let count = self.get_count();
        iter::repeat(pos).take(count as usize)
    }

    fn get_count(&self) -> i32;
}

pub trait ConditionalPlacementModifier {
    fn get_positions(
        &self,
        chunk: &ProtoChunk,
        context: &FeaturePlacementContext,
        random: &mut RandomGenerator,
        pos: BlockPos,
    ) -> Option<impl Iterator<Item = BlockPos>> {
        if self.should_place(context, chunk, random, pos) {
            return Some(iter::once(pos));
        } else {
            return None;
        }
    }

    fn should_place(
        &self,
        context: &FeaturePlacementContext,
        chunk: &ProtoChunk,
        random: &mut RandomGenerator,
        pos: BlockPos,
    ) -> bool;
}
