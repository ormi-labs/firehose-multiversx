pub mod pb;

use crate::pb::sf::multiversx::r#type::v1::HyperOutportBlock;

#[substreams::handlers::map]
pub fn map_print_block(
    blk: HyperOutportBlock,
) -> Result<HyperOutportBlock, substreams::errors::Error> {
    println!("{:?}", blk);

    Ok(blk)
}
