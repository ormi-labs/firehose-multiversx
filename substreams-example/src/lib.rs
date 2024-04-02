mod pb;

use substreams;
use substreams::errors::Error;

use pb::pbmultiversx::Block;

#[substreams::handlers::map]
pub fn map_print_block(blk: Block) -> Result<Block, Error> {
    println!("{:?}", blk);

    Ok(blk)
}
