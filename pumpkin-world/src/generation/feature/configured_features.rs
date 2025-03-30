use crate::{block::BlockStateCodec, generation::{block_state_provider::BlockStateProvider, rule_test::RuleTest}};

pub enum ConfiguredFeature {
    Ore(OreFeatureConfig),
    SimpleBlock(SimpleBlockFeatureConfig)
}

pub struct SimpleBlockFeatureConfig {
    to_place: BlockStateProvider,
    schedule_tick: Option<bool>,
}

pub struct RandomPatchFeatureConfig {
    tries: u8,
    xz_spread: u8,
    y_spread: u8
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
