mod pb;

use substreams;
use substreams::errors::Error;

use pb::multiversx;

#[substreams::handlers::map]
pub fn guardians_fn(blk: multiversx::OutportBlock) -> Result<multiversx::OutportBlock, Error> {
    let mut block_header = blk;
    block_header.shard_id = 2;

    println!("{:?}", block_header);

    Ok(block_header)
}
