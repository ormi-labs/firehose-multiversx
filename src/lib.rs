mod pb;

// use crate::pb::type_pb;

use substreams::errors::Error;
use type_pb::BlockHeader;

#[substreams::handlers::map]
fn block_index(blk: BlockHeader) -> Result<BlockHeader, Error> {
    let mut block_header = BlockHeader::default();

    Ok(block_header)
}
