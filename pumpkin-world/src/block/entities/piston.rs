use std::sync::Arc;

use async_trait::async_trait;
use pumpkin_data::block::{Block, BlockState, get_block_by_state_id, get_state_by_state_id};
use pumpkin_util::math::position::BlockPos;
use tokio::sync::Mutex;

use crate::world::{BlockFlags, SimpleWorld};

use super::BlockEntity;

pub struct PistonBlockEntity {
    pub position: BlockPos,
    pub pushed_block_state: BlockState,
    pub facing: i8,
    pub current_progress: Mutex<f32>,
    pub last_progress: Mutex<f32>,
    pub extending: bool,
    pub source: bool,
}

impl PistonBlockEntity {
    pub const ID: &'static str = "minecraft:moving_piston";
}

const FACING: &str = "facing";
const LAST_PROGRESS: &str = "progress";
const EXTENDING: &str = "extending";
const SOURCE: &str = "source";

#[async_trait]
impl BlockEntity for PistonBlockEntity {
    fn identifier(&self) -> &'static str {
        Self::ID
    }

    fn get_position(&self) -> BlockPos {
        self.position
    }

    async fn tick(&self, world: &Arc<dyn SimpleWorld>) {
        let mut last_progress = self.last_progress.lock().await;
        let mut current_progress = self.current_progress.lock().await;
        *last_progress = *current_progress;
        dbg!("ff");
        if *last_progress >= 1.0 {
            dbg!("fjfijf");
            let pos = self.position;
            world.remove_block_entity(&pos).await;
            dbg!(world.get_block(&pos).await.unwrap().name);
            if world.get_block(&pos).await.unwrap() == Block::MOVING_PISTON {
                dbg!("aa");
                if self.pushed_block_state.air {
                    world
                        .clone()
                        .set_block_state(
                            &pos,
                            self.pushed_block_state.id,
                            BlockFlags::FORCE_STATE | BlockFlags::MOVED,
                        )
                        .await;
                } else {
                    world
                        .clone()
                        .set_block_state(&pos, self.pushed_block_state.id, BlockFlags::MOVED)
                        .await;
                    world
                        .clone()
                        .update_neighbor(
                            &pos,
                            &get_block_by_state_id(self.pushed_block_state.id).unwrap(),
                        )
                        .await;
                }
            }
        }
        *current_progress = *current_progress + 0.5;
        if *current_progress >= 1.0 {
            *current_progress = 1.0;
        }
    }

    fn from_nbt(nbt: &pumpkin_nbt::compound::NbtCompound, position: BlockPos) -> Self
    where
        Self: Sized,
    {
        // TODO
        let pushed_block_state = get_state_by_state_id(Block::AIR.default_state_id).unwrap();
        let facing = nbt.get_byte(FACING).unwrap_or(0);
        let last_progress = nbt.get_float(LAST_PROGRESS).unwrap_or(0.0);
        let extending = nbt.get_bool(EXTENDING).unwrap_or(false);
        let source = nbt.get_bool(SOURCE).unwrap_or(false);
        Self {
            pushed_block_state,
            position,
            facing,
            current_progress: last_progress.into(),
            last_progress: last_progress.into(),
            extending,
            source,
        }
    }

    fn write_nbt(&self, nbt: &mut pumpkin_nbt::compound::NbtCompound) {
        // TODO: pushed_block_state
        nbt.put_byte(FACING, self.facing);
        // nbt.put_float(LAST_PROGRESS, *self.last_progress.lock().await);
        nbt.put_bool(EXTENDING, self.extending);
        nbt.put_bool(SOURCE, self.source);
    }
}
