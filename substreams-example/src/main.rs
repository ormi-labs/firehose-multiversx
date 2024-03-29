mod pb;
use pb::multiversx;

use substreams::errors::Error;

fn main() {
    println!("Hello, world!");

    let blk = multiversx::OutportBlock::default();

    let a = guardians_fn(blk);

    println!("{:?}", a);
}

fn guardians_fn(blk: multiversx::OutportBlock) -> Result<multiversx::OutportBlock, Error> {
    let mut block_header = blk;
    block_header.shard_id = 2;

    println!("{:?}", block_header);

    Ok(block_header)
}
