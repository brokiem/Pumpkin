use pumpkin_data::{block::Block, chunk::DoublePerlinNoiseParameters};
use pumpkin_util::{
    DoublePerlinNoiseParametersCodec,
    math::position::BlockPos,
    random::{RandomGenerator, RandomImpl, legacy_rand::LegacyRand},
};
use serde::Deserialize;

use crate::block::BlockStateCodec;

use super::noise::perlin::DoublePerlinNoiseSampler;

pub enum BlockStateProvider {
    NoiseThresholdBlockStateProvider(NoiseThresholdBlockStateProvider),
    NoiseProvider(NoiseBlockStateProvider),
}

impl BlockStateProvider {
    pub fn get(&self, random: &mut RandomGenerator, pos: BlockPos) -> Block {
        match self {
            BlockStateProvider::NoiseThresholdBlockStateProvider(provider) => {
                provider.get(random, pos)
            }
            BlockStateProvider::NoiseProvider(provider) => provider.get(pos),
        }
    }
}

#[derive(Deserialize)]
pub struct NoiseBlockStateProviderBase {
    seed: i64,
    noise: DoublePerlinNoiseParametersCodec,
    scale: f32,
}

fn perlin_codec_to_static(noise: DoublePerlinNoiseParametersCodec) -> DoublePerlinNoiseParameters {
    let amplitudes_static: &'static [f64] = noise.amplitudes.leak();
    DoublePerlinNoiseParameters::new(noise.first_octave, amplitudes_static, "none")
}

impl NoiseBlockStateProviderBase {
    pub fn get_noise(&self, pos: BlockPos) -> f64 {
        let noise = perlin_codec_to_static(self.noise.clone());
        let sampler = DoublePerlinNoiseSampler::new(
            &mut RandomGenerator::Legacy(LegacyRand::from_seed(self.seed as u64)),
            &noise,
            false,
        );
        sampler.sample(
            pos.0.x as f64 * self.scale as f64,
            pos.0.y as f64 * self.scale as f64,
            pos.0.z as f64 * self.scale as f64,
        )
    }
}

#[derive(Deserialize)]
pub struct NoiseBlockStateProvider {
    #[serde(flatten)]
    base: NoiseBlockStateProviderBase,
    states: Vec<BlockStateCodec>,
}

impl NoiseBlockStateProvider {
    pub fn get(&self, pos: BlockPos) -> Block {
        let value = self.base.get_noise(pos);
        self.get_state_by_value(value)
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
    low_states: Vec<BlockStateCodec>,
    high_states: Vec<BlockStateCodec>,
}

impl NoiseThresholdBlockStateProvider {
    pub fn get(&self, random: &mut RandomGenerator, pos: BlockPos) -> Block {
        let value = self.base.get_noise(pos);
        if value < self.threshold as f64 {
            return self.low_states[random.next_bounded_i32(self.low_states.len() as i32) as usize]
                .to_block();
        }
        if random.next_f32() < self.high_chance {
            return self.high_states
                [random.next_bounded_i32(self.high_states.len() as i32) as usize]
                .to_block();
        }
        self.default_state.to_block()
    }
}
