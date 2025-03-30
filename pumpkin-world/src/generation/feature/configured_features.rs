use pumpkin_util::{math::position::BlockPos, random::RandomGenerator};

use crate::{
    ProtoChunk,
    block::BlockStateCodec,
    generation::{rule_test::RuleTest},
};

use super::features::simple_block::SimpleBlockFeature;

pub enum ConfiguredFeature {
    Ore(OreFeatureConfig),
    SimpleBlock(SimpleBlockFeature),
}

impl ConfiguredFeature {
    pub fn generate(
        &self,
        chunk: &mut ProtoChunk,
        random: &mut RandomGenerator,
        pos: BlockPos,
    ) -> bool {
        match self {
            ConfiguredFeature::Ore(feature) => false, // TODO
            ConfiguredFeature::SimpleBlock(feature) => feature.generate(chunk, random, pos),
        }
    }
}

pub struct OreFeatureConfig {
    targets: Vec<Target>,
    size: u8,
    discard_chance_on_air_exposure: f32,
}

pub struct Target {
    target: RuleTest,
    state: BlockStateCodec,
}
