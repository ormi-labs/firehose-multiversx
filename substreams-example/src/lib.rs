mod pb;

use substreams;
use substreams::errors::Error;

use pb::multiversx;

#[substreams::handlers::map]
pub fn map_print_block(blk: multiversx::OutportBlock) -> Result<multiversx::OutportBlock, Error> {
    println!("{:?}", blk);

    Ok(blk)
}
