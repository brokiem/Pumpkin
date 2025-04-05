use pumpkin_data::block::Block;
use pumpkin_data::block::get_state_by_state_id;

use quote::quote;

pub(crate) fn block_state_impl(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_string = item.to_string();
    let registry_id = input_string.trim_matches('"');

    let block = Block::from_registry_key(registry_id).expect("Invalid registry id");

    let default_state_id = block.default_state_id;
    let block_id = block.id;
    let state = get_state_by_state_id(default_state_id).expect("Invalid state id");
    let air = state.air;

    if std::env::var("CARGO_PKG_NAME").unwrap() == "pumpkin-world" {
        quote! {
            crate::block::ChunkBlockState {
                state_id: #default_state_id,
                block_id: #block_id,
                air: #air,
          }
        }
        .into()
    } else {
        quote! {
            pumpkin_world::block::ChunkBlockState {
                state_id: #default_state_id,
                block_id: #block_id,
                air: #air,
            }
        }
        .into()
    }
}
