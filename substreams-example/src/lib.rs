pub mod pb;

use crate::pb::sf::multiversx::r#type::v1::HyperOutportBlock;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;

#[substreams::handlers::map]
pub fn map_print_block(
    blk: HyperOutportBlock,
) -> Result<HyperOutportBlock, substreams::errors::Error> {
    println!("{:?}", blk);

    Ok(blk)
}

#[substreams::handlers::map]
fn graph_out(blk: HyperOutportBlock) -> Result<EntityChanges, substreams::errors::Error> {
    // hash map of name to a table
    let mut tables = Tables::new();

    let blk = blk.meta_outport_block.unwrap().block_data.unwrap();
    let hex = hex::encode(&blk.header_hash);
    let header = blk.header.unwrap();

    tables
        .create_row("Block", hex)
        .set("nonce", header.nonce)
        .set("round", header.round)
        .set("epoch", header.epoch)
        .set("tx_count", header.tx_count)
        .set("timestamp", header.time_stamp)
        .set("signature", hex::encode(&header.signature))
        .set("leader_signature", hex::encode(&header.leader_signature))
        .set("pub_keys_bitmap", hex::encode(&header.pub_keys_bitmap))
        .set("prev_hash", hex::encode(&header.prev_hash))
        .set("prev_rand_seed", hex::encode(&header.prev_rand_seed))
        .set("rand_seed", hex::encode(&header.rand_seed))
        .set("root_hash", hex::encode(&header.root_hash))
        .set(
            "validator_stats_root_hash",
            hex::encode(&header.validator_stats_root_hash),
        )
        .set("receipts_hash", hex::encode(&header.receipts_hash))
        .set("chain_id", hex::encode(&header.chain_id))
        .set("software_version", hex::encode(&header.software_version))
        .set("accumulated_fees", hex::encode(&header.accumulated_fees))
        .set(
            "accumulated_fees_in_epoch",
            hex::encode(&header.accumulated_fees_in_epoch),
        )
        .set("developer_fees", hex::encode(&header.developer_fees))
        .set("dev_fees_in_epoch", hex::encode(&header.dev_fees_in_epoch));

    Ok(tables.to_entity_changes())
}
