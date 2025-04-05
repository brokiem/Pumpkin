use pumpkin_data::block::{get_block, get_state_by_state_id};

use crate::chunk::format::PaletteBlockEntry;

#[derive(Clone, Copy, Debug, Eq)]
pub struct ChunkBlockState {
    pub state_id: u16,
    pub block_id: u16,
    pub air: bool,
}

impl PartialEq for ChunkBlockState {
    fn eq(&self, other: &Self) -> bool {
        self.state_id == other.state_id
    }
}

impl ChunkBlockState {
    pub const AIR: ChunkBlockState = ChunkBlockState {
        state_id: 0,
        block_id: 0,
        air: true,
    };

    /// Get a Block from the Vanilla Block registry at Runtime
    pub fn new(registry_id: &str) -> Option<Self> {
        let block = get_block(registry_id);
        block.map(|block| Self {
            state_id: block.default_state_id,
            block_id: block.id,
            air: get_state_by_state_id(block.default_state_id).unwrap().air,
        })
    }

    pub fn from_palette(palette: &PaletteBlockEntry) -> Option<Self> {
        let block = get_block(palette.name.as_str());

        if let Some(block) = block {
            let mut state_id = block.default_state_id;
            let state = get_state_by_state_id(block.default_state_id).unwrap();

            if let Some(properties) = palette.properties.clone() {
                let mut properties_vec = Vec::new();
                for (key, value) in properties {
                    properties_vec.push((key.clone(), value.clone()));
                }
                let block_properties = block.from_properties(properties_vec).unwrap();
                state_id = block_properties.to_state_id(&block);
            }

            return Some(Self {
                state_id,
                block_id: block.id,
                air: state.air,
            });
        }

        None
    }

    pub fn get_id(&self) -> u16 {
        self.state_id
    }

    #[inline]
    pub fn is_air(&self) -> bool {
        self.air
    }

    #[inline]
    pub fn of_block(&self, block_id: u16) -> bool {
        self.block_id == block_id
    }
}

#[cfg(test)]
mod tests {
    use super::ChunkBlockState;

    #[test]
    fn not_existing() {
        let result = ChunkBlockState::new("this_block_does_not_exist");
        assert!(result.is_none());
    }

    #[test]
    fn does_exist() {
        let result = ChunkBlockState::new("dirt");
        assert!(result.is_some());
    }
}
