pub mod pb;

pub mod decode;

use crate::decode::parse_data;
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

mod methods {
    /// https://github.com/multiversx/mx-specs/blob/main/ESDT-specs.md#issuance-of-fungible-esdt-tokens
    pub const ISSUANCE: &str = "issue";
    pub const TRANSFER: &str = "ESDTTransfer";
    pub const ISSUANCE_NFT: &str = "issueNonFungible";
    pub const ISSUANCE_SFT: &str = "issueSemiFungible";
    pub const SET_ROLE: &str = "setSpecialRole";
    pub const CREATE_ROLE: &str = "ESDTNFTCreateRoleTransfer";
    pub const CREATE_NFT: &str = "ESDTNFTCreate";
    pub const TRANSFER_NFT: &str = "ESDTNFTTransfer";
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


        match method {
            methods::ISSUANCE => {
                parse_data(segments, methods::ISSUANCE, |mut p| {
                    tables
                        .create_row("IssuanceTransaction", id.clone())
                        .set("tx", id.clone())
                        .set("token_name", p.next_utf8("token_name"))
                        .set("token_ticker", p.next_utf8("token_ticker"))
                        .set("initial_supply", p.next_bigint("initial_supply"))
                        .set("decimals", p.next_bigint("decimals"))
                        .set("extra_fields", p.extra_fields());
                });
            }
            methods::TRANSFER => {
                parse_data(segments, methods::TRANSFER, |mut p| {
                    let row = tables
                        .create_row("TransferTransaction", id.clone())
                        .set("tx", id.clone())
                        .set("token_identifier", p.next_utf8("token_identifier"))
                        .set("value", p.next_bigint("value"));
                    // https://github.com/multiversx/mx-specs/blob/main/ESDT-specs.md#transfers-to-a-smart-contract
                    if p.has_next() {
                        row.set("method", p.next_utf8("method"))
                            .set("arg1", p.next_raw("arg1"))
                            .set("arg2", p.next_raw("arg2"))
                            .set("extra_fields", p.extra_fields());
                    }
                });
            }
            methods::ISSUANCE_NFT => parse_data(segments, methods::ISSUANCE_NFT, |mut p| {
                tables
                    .create_row("NFTIssuanceTransaction", id.clone())
                    .set("tx", id.clone())
                    .set("token_name", p.next_utf8("token_name"))
                    .set("token_ticker", p.next_utf8("token_ticker"))
                    .set("extra_fields", p.extra_fields());
            }),
            methods::ISSUANCE_SFT => parse_data(segments, methods::ISSUANCE_SFT, |mut p| {
                tables
                    .create_row("SFTIssuanceTransaction", id.clone())
                    .set("tx", id.clone())
                    .set("token_name", p.next_utf8("token_name"))
                    .set("token_ticker", p.next_utf8("token_ticker"))
                    .set("extra_fields", p.extra_fields());
            }),
            methods::SET_ROLE => parse_data(segments, methods::SET_ROLE, |mut p| {
                let row = tables
                    .create_row("RolesAssigningTransaction", id.clone())
                    .set("tx", id.clone())
                    .set("token_identifier", p.next_utf8("token_identifier"))
                    .set("address", p.next_utf8("address"));

                let mut roles = vec![];
                while p.has_next() {
                    roles.push(p.next_utf8("role"));
                }

                row.set("roles", roles);
            }),
            methods::CREATE_NFT => parse_data(segments, methods::CREATE_NFT, |mut p| {
                let row = tables
                    .create_row("NFTCreationTransaction", id.clone())
                    .set("tx", id.clone())
                    .set("token_identifier", p.next_utf8("token_identifier"))
                    .set("initial_quantity", p.next_bigint("initial_quantity"))
                    .set("nft_name", p.next_utf8("nft_name"))
                    .set("royalties", p.next_bigint("royalties"))
                    .set("hash", p.next_utf8("hash"))
                    .set("attributes", p.next_raw("attributes"));

                let mut uris = vec![];
                while p.has_next() {
                    uris.push(p.next_utf8("uri"));
                }

                row.set("uris", uris);
            }),
            methods::CREATE_ROLE => parse_data(segments, methods::CREATE_ROLE, |mut p| {
                tables
                    .create_row("TransferCreationRoleTransaction", id.clone())
                    .set("tx", id.clone())
                    .set("token_identifier", p.next_utf8("token_identifier"))
                    .set("address_from", p.next_utf8("address_from"))
                    .set("address_to", p.next_utf8("address_to"));
            }),
            methods::TRANSFER_NFT => parse_data(segments, methods::TRANSFER_NFT, |mut p| {
                tables
                    .create_row("NFTTransferTransaction", id.clone())
                    .set("tx", id.clone())
                    .set("token_identifier", p.next_utf8("token_identifier"))
                    .set("nonce", p.next_bigint("nonce"))
                    .set("quantity", p.next_bigint("quantity"))
                    .set("destination", p.next_utf8("destination"));
            }),
            _ => {}
        }
    }

    Ok(tables.to_entity_changes())
}
