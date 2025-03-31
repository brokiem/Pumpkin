use itertools::Itertools;
use pumpkin_data::block::Block;
use serde::Deserialize;
#[derive(Deserialize)]
pub struct EmptyTODOStruct {}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum BlockPredicate {
    #[serde(rename = "minecraft:matching_blocks")]
    MatchingBlocksBlockPredicate(MatchingBlocksBlockPredicate),
    #[serde(rename = "minecraft:matching_block_tag")]
    MatchingBlockTagPredicate(EmptyTODOStruct),
    #[serde(rename = "minecraft:matching_fluids")]
    MatchingFluidsBlockPredicate(EmptyTODOStruct),
    #[serde(rename = "minecraft:has_sturdy_face")]
    HasSturdyFacePredicate(EmptyTODOStruct),
    #[serde(rename = "minecraft:solid")]
    SolidBlockPredicate(EmptyTODOStruct),
    #[serde(rename = "minecraft:replaceable")]
    ReplaceableBlockPredicate(EmptyTODOStruct),
    #[serde(rename = "minecraft:would_survive")]
    WouldSurviveBlockPredicate(EmptyTODOStruct),
    #[serde(rename = "minecraft:inside_world_bounds")]
    InsideWorldBoundsBlockPredicate(EmptyTODOStruct),
    #[serde(rename = "minecraft:any_of")]
    AnyOfBlockPredicate(EmptyTODOStruct),
    #[serde(rename = "minecraft:all_of")]
    AllOfBlockPredicate(EmptyTODOStruct),
    #[serde(rename = "minecraft:not")]
    NotBlockPredicate(EmptyTODOStruct),
    #[serde(rename = "minecraft:true")]
    AlwaysTrueBlockPredicate(EmptyTODOStruct),
    #[serde(rename = "minecraft:unobstructed")]
    UnobstructedBlockPredicate(EmptyTODOStruct),
}

impl BlockPredicate {
    pub fn test(&self, block: &Block) -> bool {
        match self {
            BlockPredicate::MatchingBlocksBlockPredicate(predicate) => predicate.test(block),
            _ => false,
        }
    }
}

#[derive(Deserialize)]
pub struct MatchingBlocksBlockPredicate {
    blocks: MatchingBlocksWrapper,
}

impl MatchingBlocksBlockPredicate {
    pub fn test(&self, block: &Block) -> bool {
        match &self.blocks {
            MatchingBlocksWrapper::Single(single_block) => {
                single_block.replace("minecraft:", "") == block.name
            }
            MatchingBlocksWrapper::Multiple(blocks) => blocks
                .iter()
                .map(|s| s.replace("minecraft:", ""))
                .contains(block.name),
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum MatchingBlocksWrapper {
    Single(String),
    Multiple(Vec<String>),
}
