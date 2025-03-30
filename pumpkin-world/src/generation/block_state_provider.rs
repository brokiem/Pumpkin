use pumpkin_data::{block::Block, chunk::DoublePerlinNoiseParameters};
use pumpkin_util::{math::position::BlockPos, random::xoroshiro128::Xoroshiro, DoublePerlinNoiseParametersCodec};
use serde::Deserialize;

use crate::block::BlockStateCodec;

use super::noise::perlin::DoublePerlinNoiseSampler;

pub enum BlockStateProvider {
    NoiseThresholdBlockStateProvider(NoiseThresholdBlockStateProvider),
    NoiseProvider(NoiseBlockStateProvider)
}

#[derive(Deserialize)]
pub struct NoiseBlockStateProviderBase {
    seed: i64,
    noise: DoublePerlinNoiseParametersCodec,
    scale: f32,
}

fn perlin_codec_to_static(noise: DoublePerlinNoiseParametersCodec) -> DoublePerlinNoiseParameters {
    DoublePerlinNoiseParameters::new(noise.first_octave, &noise.amplitudes, "none")
}

impl NoiseBlockStateProviderBase {
    pub fn get_noise(&self, pos: BlockPos, scale: f64) {
        let sampler = DoublePerlinNoiseSampler::new(Xoroshiro::from_seed_unmixed(self.seed as u64), self.noise, false);
    }
    
}

#[derive(Deserialize)]
pub struct NoiseBlockStateProvider {
    #[serde(flatten)]
    base: NoiseBlockStateProviderBase,
    states: Vec<BlockStateCodec>
}

impl NoiseBlockStateProvider {
    pub fn get() -> Block {
    }
    
    fn get_state_by_value(&self, value: f64) -> Block {
        let val = ((1.0 + value) / 2.0).clamp(0.0, 0.9999);
        self.states[(val * self.states.len() as f64) as usize].to_block()
    }
}

#[derive(Deserialize)]
pub struct NoiseThresholdBlockStateProvider {
    #[serde(flatten)]
    base: NoiseBlockStateProviderBase,
    threshold: f32,
    high_chance: f32,
    default_state: BlockStateCodec,
    high_states: Vec<BlockStateCodec>
}