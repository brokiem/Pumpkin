use pumpkin_util::{math::position::BlockPos, random::RandomGenerator};

use crate::{
    ProtoChunk, block::ChunkBlockState, generation::block_state_provider::BlockStateProvider,
};

pub struct SimpleBlockFeature {
    to_place: BlockStateProvider,
    schedule_tick: Option<bool>,
}

impl SimpleBlockFeature {
    pub fn generate(
        &self,
        chunk: &mut ProtoChunk,
        random: &mut RandomGenerator,
        pos: BlockPos,
    ) -> bool {
        let block = self.to_place.get(random, pos);
        // TODO: check things..
        chunk.set_block_state(
            &pos.0,
            ChunkBlockState {
                state_id: block.default_state_id,
                block_id: block.id,
            },
        );
        // TODO: scheudle tick when needed
        true
    }
}
