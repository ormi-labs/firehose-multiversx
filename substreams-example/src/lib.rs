pub mod pb;

pub mod decode;
pub mod utils;

use crate::decode::parse_data;
use crate::pb::sf::multiversx::r#type::v1::HyperOutportBlock;
use crate::utils::{Field, RowExt};
use substreams::hex;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;

#[substreams::handlers::map]
pub fn map_print_block(
    blk: HyperOutportBlock,
) -> Result<HyperOutportBlock, substreams::errors::Error> {
    println!("{:?}", blk);

    Ok(blk)
}

/// Address of the system smart contract that manages ESDT.
const SYSTEM_SC_ADDRESS_BYTES: [u8; 32] =
    hex!("000000000000000000010000000000000000000000000000000000000002ffff");

mod methods {
    /// https://github.com/multiversx/mx-specs/blob/main/ESDT-specs.md#issuance-of-fungible-esdt-tokens
    pub const ISSUANCE: &str = "issue";
    pub const TRANSFER: &str = "ESDTTransfer";
}

#[substreams::handlers::map]
fn graph_out(blk: HyperOutportBlock) -> Result<EntityChanges, substreams::errors::Error> {
    // hash map of name to a table
    let mut tables = Tables::new();

    let outport = blk.meta_outport_block.unwrap();
    let blk = outport.block_data.unwrap();
    // let hex = hex::encode(&blk.header_hash);
    let header = blk.header.unwrap();

    let tx_pool = outport.transaction_pool.unwrap();
    for (id, tx) in tx_pool.transactions {
        let tx = tx.transaction.expect("Should have tx");

        tables
            .create_row("Transaction", id.clone())
            .set("timestamp", header.time_stamp)
            .set("sender", hex::encode(&tx.snd_addr))
            .set("receiver", hex::encode(&tx.rcv_addr))
            .set("data", hex::encode(&tx.data));

        if tx.rcv_addr != SYSTEM_SC_ADDRESS_BYTES {
            continue;
        }

        let utf_data = String::from_utf8(tx.data.to_vec())
            .expect("Failed to parse `IssuanceTransaction` data as a string");
        let mut segments = utf_data.split('@');

        let method = segments.next().expect("Should have method");

        let tx_field = Field::from_tovalue("tx", id.clone());

        match method {
            methods::ISSUANCE => {
                parse_data(segments, methods::ISSUANCE, |mut p| {
                    tables
                        .create_row("IssuanceTransaction", id.clone())
                        .set_field(tx_field)
                        .set_field(p.field_string("token_name"))
                        .set_field(p.field_string("token_ticker"))
                        .set_field(p.field_bigint("initial_supply"))
                        .set_field(p.field_bigint("decimals"))
                        .set_field(p.extra_fields());
                });
            }
            methods::TRANSFER => {
                parse_data(segments, methods::TRANSFER, |mut p| {
                    let row = tables
                        .create_row("TransferTransaction", id.clone())
                        .set_field(tx_field)
                        .set_field(p.field_string("token_identifier"))
                        .set_field(p.field_bigint("value"));
                    // https://github.com/multiversx/mx-specs/blob/main/ESDT-specs.md#transfers-to-a-smart-contract
                    if p.has_next() {
                        row.set_field(p.field_string("method"))
                            .set_field(p.field_raw("arg1"))
                            .set_field(p.field_raw("arg2"))
                            .set_field(p.extra_fields());
                    }
                });
            }
            _ => {}
        }
    }

    Ok(tables.to_entity_changes())
}
