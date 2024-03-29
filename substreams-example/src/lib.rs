mod pb;

use substreams;
use substreams::errors::Error;

use pb::sf::multiversx::r#type::v1::OutportBlock;

#[substreams::handlers::map]
pub fn map_print_block(blk: OutportBlock) -> Result<OutportBlock, Error> {
    println!("{:?}", blk);

    Ok(blk)
}
