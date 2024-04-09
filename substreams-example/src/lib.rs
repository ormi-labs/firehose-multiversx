pub mod pb;

use crate::pb::pbmultiversx::Block;

#[substreams::handlers::map]
pub fn map_print_block(blk: Block) -> Result<Block, substreams::errors::Error> {
    println!("{:?}", blk);

    Ok(blk)
}
