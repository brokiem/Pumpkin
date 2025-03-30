use pumpkin_util::{
    math::{position::BlockPos, vector3::Vector3},
    random::RandomGenerator,
};

use crate::{ProtoChunk, generation::feature::configured_features::ConfiguredFeature};

pub struct RandomPatchFeature {
    tries: u8,
    xz_spread: u8,
    y_spread: u8,
    feature: ConfiguredFeature,
}

impl RandomPatchFeature {
    pub fn generate(
        &self,
        chunk: &mut ProtoChunk,
        random: &mut RandomGenerator,
        pos: BlockPos,
    ) -> bool {
        let mut i = 0;
        let xz = self.xz_spread as i32 + 1;
        let y = self.y_spread as i32 + 1;
        for _ in 0..self.tries {
            let pos = Vector3::new(
                pos.0.x + (random.next_bounded_i32(xz) - random.next_bounded_i32(xz)),
                pos.0.y + (random.next_bounded_i32(y) - random.next_bounded_i32(y)),
                pos.0.z + (random.next_bounded_i32(xz) - random.next_bounded_i32(xz)),
            );
            if !self.feature.generate(chunk, random, BlockPos(pos)) {
                continue;
            }
            i += 1;
        }
        i > 0
    }
}
