mod block_with_hash;
pub use block_with_hash::{BlockHeaderWithHash, BlockWithHash};

mod applied_block;
pub use applied_block::AppliedBlock;

pub mod prevalidate;

pub mod genesis;

use std::sync::Arc;

pub use mina_p2p_messages::v2::{
    MinaBlockBlockStableV2 as Block, MinaBlockHeaderStableV2 as BlockHeader, StateHash as BlockHash,
};

pub type ArcBlock = Arc<Block>;
pub type ArcBlockWithHash = BlockWithHash<Arc<Block>>;
