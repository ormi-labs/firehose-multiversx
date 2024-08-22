pub mod pb;

use crate::pb::sf::multiversx::r#type::v1::HyperOutportBlock;
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
        /// https://github.com/multiversx/mx-specs/blob/main/ESDT-specs.md#issuance-of-fungible-esdt-tokens
        const ISSUANCE_TRANSACTION_PREFIX: &[u8] = b"issue";

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

        if tx.data.starts_with(ISSUANCE_TRANSACTION_PREFIX) {
            let utf_data = String::from_utf8(tx.data.clone())
                .expect("Failed to parse `IssuanceTransaction` data as a string");

            let mut split = utf_data.split("@");
            let _issue = split.next().expect("Should have issue segment");
            let token_name = split
                .next()
                .map(|s| {
                    String::from_utf8(hex::decode(s).expect("token_name should be decodable"))
                        .expect("token_name should be a string")
                })
                .expect("Should stringify token_name segment");
            let token_ticker = split
                .next()
                .map(|s| {
                    String::from_utf8(hex::decode(s).expect("token_ticker should be decodable"))
                        .expect("token_name should be a string")
                })
                .expect("Should stringify token_ticker segment");
            let initial_supply = split.next().expect("Should have initial_supply segment");
            let decimals = split.next().expect("Should have decimals segment");
            let extra_fields = split
                .map(|s| {
                    String::from_utf8(hex::decode(s).expect("should decode other fields")).unwrap()
                })
                .collect::<Vec<_>>()
                .chunks_exact(2)
                .map(|c| format!("{}: {}", c[0], c[1]))
                .collect::<Vec<_>>()
                .join(", ");

            let token_manager_addr = &tx.snd_addr;
            tables
                .create_row("IssuanceTransaction", id.clone())
                .set("tx", id.clone())
                .set("token_manager_addr", hex::encode(token_manager_addr))
                .set("token_name", token_name)
                .set("token_ticker", token_ticker)
                .set("initial_supply", initial_supply)
                .set("decimals", decimals)
                .set("extra_fields", extra_fields);
        }
    }

    Ok(tables.to_entity_changes())
}
