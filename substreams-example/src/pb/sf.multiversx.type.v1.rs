// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HyperOutportBlock {
    #[prost(message, optional, tag="1")]
    pub meta_outport_block: ::core::option::Option<MetaOutportBlock>,
    #[prost(message, repeated, tag="2")]
    pub notarized_headers_outport_data: ::prost::alloc::vec::Vec<NotarizedHeaderOutportData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotarizedHeaderOutportData {
    #[prost(string, tag="1")]
    pub notarized_header_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub outport_block: ::core::option::Option<ShardOutportBlock>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHashRequest {
    #[prost(string, tag="1")]
    pub hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockNonceRequest {
    #[prost(uint64, tag="1")]
    pub nonce: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetaOutportBlock {
    #[prost(uint32, tag="1")]
    pub shard_id: u32,
    #[prost(message, optional, tag="2")]
    pub block_data: ::core::option::Option<MetaBlockData>,
    #[prost(message, optional, tag="3")]
    pub transaction_pool: ::core::option::Option<TransactionPool>,
    #[prost(message, optional, tag="4")]
    pub header_gas_consumption: ::core::option::Option<HeaderGasConsumption>,
    #[prost(map="string, message", tag="5")]
    pub altered_accounts: ::std::collections::HashMap<::prost::alloc::string::String, AlteredAccount>,
    #[prost(string, repeated, tag="6")]
    pub notarized_headers_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag="7")]
    pub number_of_shards: u32,
    #[prost(uint64, repeated, tag="8")]
    pub signers_indexes: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, tag="9")]
    pub highest_final_block_nonce: u64,
    #[prost(bytes="vec", tag="10")]
    pub highest_final_block_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardOutportBlock {
    #[prost(uint32, tag="1")]
    pub shard_id: u32,
    #[prost(message, optional, tag="2")]
    pub block_data: ::core::option::Option<BlockData>,
    #[prost(message, optional, tag="3")]
    pub transaction_pool: ::core::option::Option<TransactionPool>,
    #[prost(message, optional, tag="4")]
    pub header_gas_consumption: ::core::option::Option<HeaderGasConsumption>,
    #[prost(map="string, message", tag="5")]
    pub altered_accounts: ::std::collections::HashMap<::prost::alloc::string::String, AlteredAccount>,
    #[prost(string, repeated, tag="6")]
    pub notarized_headers_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag="7")]
    pub number_of_shards: u32,
    #[prost(uint64, repeated, tag="8")]
    pub signers_indexes: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, tag="9")]
    pub highest_final_block_nonce: u64,
    #[prost(bytes="vec", tag="10")]
    pub highest_final_block_hash: ::prost::alloc::vec::Vec<u8>,
}
/// Header holds the metadata of a block. This is the part that is being hashed and run through consensus.
/// The header holds the hash of the body and also the link to the previous block header hash
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    #[prost(uint64, tag="1")]
    pub nonce: u64,
    #[prost(bytes="vec", tag="2")]
    pub prev_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub prev_rand_seed: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub rand_seed: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub pub_keys_bitmap: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="6")]
    pub shard_id: u32,
    #[prost(uint64, tag="7")]
    pub time_stamp: u64,
    #[prost(uint64, tag="8")]
    pub round: u64,
    #[prost(uint32, tag="9")]
    pub epoch: u32,
    #[prost(enumeration="Type", tag="10")]
    pub block_body_type: i32,
    #[prost(bytes="vec", tag="11")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="12")]
    pub leader_signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="13")]
    pub mini_block_headers: ::prost::alloc::vec::Vec<MiniBlockHeader>,
    #[prost(message, repeated, tag="14")]
    pub peer_changes: ::prost::alloc::vec::Vec<PeerChange>,
    #[prost(bytes="vec", tag="15")]
    pub root_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="16")]
    pub meta_block_hashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, tag="17")]
    pub tx_count: u32,
    #[prost(bytes="vec", tag="18")]
    pub epoch_start_meta_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="19")]
    pub receipts_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="20")]
    pub chain_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="21")]
    pub software_version: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="22")]
    pub accumulated_fees: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="23")]
    pub developer_fees: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="24")]
    pub reserved: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetaHeader {
    #[prost(uint64, tag="1")]
    pub nonce: u64,
    #[prost(uint32, tag="2")]
    pub epoch: u32,
    #[prost(uint64, tag="3")]
    pub round: u64,
    #[prost(uint64, tag="4")]
    pub time_stamp: u64,
    #[prost(message, repeated, tag="5")]
    pub shard_info: ::prost::alloc::vec::Vec<ShardData>,
    #[prost(message, repeated, tag="6")]
    pub peer_info: ::prost::alloc::vec::Vec<PeerData>,
    #[prost(bytes="vec", tag="7")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub leader_signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="9")]
    pub pub_keys_bitmap: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="10")]
    pub prev_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="11")]
    pub prev_rand_seed: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="12")]
    pub rand_seed: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="13")]
    pub root_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="14")]
    pub validator_stats_root_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="16")]
    pub mini_block_headers: ::prost::alloc::vec::Vec<MiniBlockHeader>,
    #[prost(bytes="vec", tag="17")]
    pub receipts_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="18")]
    pub epoch_start: ::core::option::Option<EpochStart>,
    #[prost(bytes="vec", tag="19")]
    pub chain_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="20")]
    pub software_version: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="21")]
    pub accumulated_fees: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="22")]
    pub accumulated_fees_in_epoch: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="23")]
    pub developer_fees: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="24")]
    pub dev_fees_in_epoch: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="25")]
    pub tx_count: u32,
    #[prost(bytes="vec", tag="26")]
    pub reserved: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MiniBlockHeader {
    #[prost(bytes="vec", tag="1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="2")]
    pub sender_shard_id: u32,
    #[prost(uint32, tag="3")]
    pub receiver_shard_id: u32,
    #[prost(uint32, tag="4")]
    pub tx_count: u32,
    #[prost(enumeration="Type", tag="5")]
    pub r#type: i32,
    #[prost(bytes="vec", tag="6")]
    pub reserved: ::prost::alloc::vec::Vec<u8>,
}
/// PeerChange holds a change in one peer to shard assignation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerChange {
    #[prost(bytes="vec", tag="1")]
    pub pub_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="2")]
    pub shard_id_dest: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockData {
    #[prost(uint32, tag="1")]
    pub shard_id: u32,
    #[prost(message, optional, tag="2")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag="3")]
    pub header_type: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="4")]
    pub header_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="5")]
    pub body: ::core::option::Option<Body>,
    #[prost(message, repeated, tag="6")]
    pub intra_shard_mini_blocks: ::prost::alloc::vec::Vec<MiniBlock>,
    #[prost(bytes="vec", tag="7")]
    pub scheduled_root_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub scheduled_accumulated_fees: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="9")]
    pub scheduled_developer_fees: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="10")]
    pub scheduled_gas_provided: u64,
    #[prost(uint64, tag="11")]
    pub scheduled_gas_penalized: u64,
    #[prost(uint64, tag="12")]
    pub scheduled_gas_refunded: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetaBlockData {
    #[prost(uint32, tag="1")]
    pub shard_id: u32,
    #[prost(message, optional, tag="2")]
    pub header: ::core::option::Option<MetaHeader>,
    #[prost(string, tag="3")]
    pub header_type: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="4")]
    pub header_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="5")]
    pub body: ::core::option::Option<Body>,
    #[prost(message, repeated, tag="6")]
    pub intra_shard_mini_blocks: ::prost::alloc::vec::Vec<MiniBlock>,
    #[prost(bytes="vec", tag="7")]
    pub scheduled_root_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub scheduled_accumulated_fees: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="9")]
    pub scheduled_developer_fees: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="10")]
    pub scheduled_gas_provided: u64,
    #[prost(uint64, tag="11")]
    pub scheduled_gas_penalized: u64,
    #[prost(uint64, tag="12")]
    pub scheduled_gas_refunded: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardData {
    #[prost(uint32, tag="1")]
    pub shard_id: u32,
    #[prost(bytes="vec", tag="2")]
    pub header_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="3")]
    pub shard_mini_block_headers: ::prost::alloc::vec::Vec<MiniBlockHeader>,
    #[prost(bytes="vec", tag="4")]
    pub prev_rand_seed: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub pub_keys_bitmap: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="7")]
    pub tx_count: u32,
    #[prost(uint64, tag="8")]
    pub round: u64,
    #[prost(bytes="vec", tag="9")]
    pub prev_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="10")]
    pub nonce: u64,
    #[prost(uint32, tag="11")]
    pub num_pending_mini_blocks: u32,
    #[prost(bytes="vec", tag="12")]
    pub accumulated_fees: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="14")]
    pub developer_fees: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="13")]
    pub last_included_meta_nonce: u64,
}
/// EpochStart holds the block information for end-of-epoch
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpochStart {
    #[prost(message, repeated, tag="1")]
    pub last_finalized_headers: ::prost::alloc::vec::Vec<EpochStartShardData>,
    #[prost(message, optional, tag="2")]
    pub economics: ::core::option::Option<Economics>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpochStartShardData {
    #[prost(uint32, tag="1")]
    pub shard_id: u32,
    #[prost(bytes="vec", tag="2")]
    pub header_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub root_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub first_pending_meta_block: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub last_finished_meta_block: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="6")]
    pub pending_mini_block_headers: ::prost::alloc::vec::Vec<MiniBlockHeader>,
    #[prost(uint64, tag="7")]
    pub round: u64,
    #[prost(uint64, tag="8")]
    pub nonce: u64,
    #[prost(uint32, tag="9")]
    pub epoch: u32,
    #[prost(bytes="vec", tag="10")]
    pub scheduled_root_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Economics {
    #[prost(bytes="vec", tag="1")]
    pub total_supply: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub total_to_distribute: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub total_newly_minted: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub rewards_per_block: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub rewards_for_protocol_sustainability: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub node_price: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="7")]
    pub prev_epoch_start_round: u64,
    #[prost(bytes="vec", tag="8")]
    pub prev_epoch_start_hash: ::prost::alloc::vec::Vec<u8>,
}
/// PeerData holds information about actions taken by a peer:
///   - a peer can register with an amount to become a validator
///   - a peer can choose to deregister and get back the deposited value
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerData {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration="PeerAction", tag="3")]
    pub action: i32,
    #[prost(uint64, tag="4")]
    pub time_stamp: u64,
    #[prost(bytes="vec", tag="5")]
    pub value_change: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionPool {
    #[prost(map="string, message", tag="1")]
    pub transactions: ::std::collections::HashMap<::prost::alloc::string::String, TxInfo>,
    #[prost(map="string, message", tag="2")]
    pub smart_contract_results: ::std::collections::HashMap<::prost::alloc::string::String, ScrInfo>,
    #[prost(map="string, message", tag="3")]
    pub rewards: ::std::collections::HashMap<::prost::alloc::string::String, RewardInfo>,
    #[prost(map="string, message", tag="4")]
    pub receipts: ::std::collections::HashMap<::prost::alloc::string::String, Receipt>,
    #[prost(map="string, message", tag="5")]
    pub invalid_txs: ::std::collections::HashMap<::prost::alloc::string::String, TxInfo>,
    #[prost(message, repeated, tag="6")]
    pub logs: ::prost::alloc::vec::Vec<LogData>,
    #[prost(string, repeated, tag="7")]
    pub scheduled_executed_scrs_hashes_prev_block: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="8")]
    pub scheduled_executed_invalid_txs_hashes_prev_block: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeInfo {
    #[prost(uint64, tag="1")]
    pub gas_used: u64,
    #[prost(bytes="vec", tag="2")]
    pub fee: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub initial_paid_fee: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxInfo {
    #[prost(message, optional, tag="1")]
    pub transaction: ::core::option::Option<Transaction>,
    #[prost(message, optional, tag="2")]
    pub fee_info: ::core::option::Option<FeeInfo>,
    #[prost(uint32, tag="3")]
    pub execution_order: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScrInfo {
    #[prost(message, optional, tag="1")]
    pub smart_contract_result: ::core::option::Option<SmartContractResult>,
    #[prost(message, optional, tag="2")]
    pub fee_info: ::core::option::Option<FeeInfo>,
    #[prost(uint32, tag="3")]
    pub execution_order: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogData {
    #[prost(string, tag="1")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub log: ::core::option::Option<Log>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardInfo {
    #[prost(message, optional, tag="1")]
    pub reward: ::core::option::Option<RewardTx>,
    #[prost(uint32, tag="2")]
    pub execution_order: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderGasConsumption {
    #[prost(uint64, tag="1")]
    pub gas_provided: u64,
    #[prost(uint64, tag="2")]
    pub gas_refunded: u64,
    #[prost(uint64, tag="3")]
    pub gas_penalized: u64,
    #[prost(uint64, tag="4")]
    pub max_gas_per_block: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorRatingInfo {
    #[prost(string, tag="1")]
    pub public_key: ::prost::alloc::string::String,
    #[prost(float, tag="2")]
    pub rating: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorsRating {
    #[prost(uint32, tag="1")]
    pub shard_id: u32,
    #[prost(uint32, tag="2")]
    pub epoch: u32,
    #[prost(message, repeated, tag="3")]
    pub validators_rating_info: ::prost::alloc::vec::Vec<ValidatorRatingInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoundInfo {
    #[prost(uint64, tag="1")]
    pub round: u64,
    #[prost(uint64, repeated, tag="2")]
    pub signers_indexes: ::prost::alloc::vec::Vec<u64>,
    #[prost(bool, tag="3")]
    pub block_was_proposed: bool,
    #[prost(uint32, tag="4")]
    pub shard_id: u32,
    #[prost(uint32, tag="5")]
    pub epoch: u32,
    #[prost(uint64, tag="6")]
    pub timestamp: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoundsInfo {
    #[prost(uint32, tag="1")]
    pub shard_id: u32,
    #[prost(message, repeated, tag="2")]
    pub rounds_info: ::prost::alloc::vec::Vec<RoundInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubKeys {
    #[prost(bytes="vec", repeated, tag="1")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorsPubKeys {
    #[prost(uint32, tag="1")]
    pub shard_id: u32,
    #[prost(map="uint32, message", tag="2")]
    pub shard_validators_pub_keys: ::std::collections::HashMap<u32, PubKeys>,
    #[prost(uint32, tag="3")]
    pub epoch: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Accounts {
    #[prost(uint32, tag="1")]
    pub shard_id: u32,
    #[prost(uint64, tag="2")]
    pub block_timestamp: u64,
    #[prost(map="string, message", tag="3")]
    pub altered_accounts: ::std::collections::HashMap<::prost::alloc::string::String, AlteredAccount>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizedBlock {
    #[prost(uint32, tag="1")]
    pub shard_id: u32,
    #[prost(bytes="vec", tag="2")]
    pub header_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shard {
    #[prost(uint32, tag="1")]
    pub shard_id: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Body {
    #[prost(message, repeated, tag="1")]
    pub mini_blocks: ::prost::alloc::vec::Vec<MiniBlock>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MiniBlock {
    #[prost(bytes="vec", repeated, tag="1")]
    pub tx_hashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, tag="2")]
    pub receiver_shard_id: u32,
    #[prost(uint32, tag="3")]
    pub sender_shard_id: u32,
    #[prost(enumeration="Type", tag="4")]
    pub r#type: i32,
    #[prost(bytes="vec", tag="5")]
    pub reserved: ::prost::alloc::vec::Vec<u8>,
}
/// Transaction holds all the data needed for a value transfer or SC call
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(uint64, tag="1")]
    pub nonce: u64,
    #[prost(bytes="vec", tag="2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub rcv_addr: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub rcv_user_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub snd_addr: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub snd_user_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="7")]
    pub gas_price: u64,
    #[prost(uint64, tag="8")]
    pub gas_limit: u64,
    #[prost(bytes="vec", tag="9")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="10")]
    pub chain_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="11")]
    pub version: u32,
    #[prost(bytes="vec", tag="12")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="13")]
    pub options: u32,
    #[prost(bytes="vec", tag="14")]
    pub guardian_addr: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="15")]
    pub guardian_signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmartContractResult {
    #[prost(uint64, tag="1")]
    pub nonce: u64,
    #[prost(bytes="vec", tag="2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub rcv_addr: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub snd_addr: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub relayer_addr: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub relayed_value: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub code: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="9")]
    pub prev_tx_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="10")]
    pub original_tx_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="11")]
    pub gas_limit: u64,
    #[prost(uint64, tag="12")]
    pub gas_price: u64,
    #[prost(int64, tag="13")]
    pub call_type: i64,
    #[prost(bytes="vec", tag="14")]
    pub code_metadata: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="15")]
    pub return_message: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="16")]
    pub original_sender: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Log {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="2")]
    pub events: ::prost::alloc::vec::Vec<Event>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub identifier: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="3")]
    pub topics: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="5")]
    pub additional_data: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardTx {
    #[prost(uint64, tag="1")]
    pub round: u64,
    #[prost(bytes="vec", tag="3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub rcv_addr: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="2")]
    pub epoch: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlteredAccount {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub nonce: u64,
    #[prost(string, tag="3")]
    pub balance: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub tokens: ::prost::alloc::vec::Vec<AccountTokenData>,
    #[prost(message, optional, tag="5")]
    pub additional_data: ::core::option::Option<AdditionalAccountData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountTokenData {
    #[prost(uint64, tag="1")]
    pub nonce: u64,
    #[prost(string, tag="2")]
    pub identifier: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub balance: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub properties: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub meta_data: ::core::option::Option<TokenMetaData>,
    #[prost(message, optional, tag="6")]
    pub additional_data: ::core::option::Option<AdditionalAccountTokenData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenMetaData {
    #[prost(uint64, tag="1")]
    pub nonce: u64,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub creator: ::prost::alloc::string::String,
    #[prost(uint32, tag="4")]
    pub royalties: u32,
    #[prost(bytes="vec", tag="5")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="6")]
    pub ur_is: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="7")]
    pub attributes: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionalAccountTokenData {
    #[prost(bool, tag="1")]
    pub is_nft_create: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionalAccountData {
    #[prost(bool, tag="1")]
    pub is_sender: bool,
    #[prost(bool, tag="2")]
    pub balance_changed: bool,
    #[prost(string, tag="3")]
    pub current_owner: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub developer_rewards: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub code_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub root_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub code_metadata: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Receipt {
    #[prost(bytes="vec", tag="1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub snd_addr: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub tx_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigInt {
    #[prost(bytes="vec", tag="1")]
    pub bytes: ::prost::alloc::vec::Vec<u8>,
}
/// PeerAction type represents the possible events that a node can trigger for the metachain to notarize
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PeerAction {
    InvalidAction = 0,
    PeerRegistration = 1,
    PeerUnstaking = 2,
    PeerDeregistration = 3,
    PeerJailed = 4,
    PeerUnJailed = 5,
    PeerSlashed = 6,
    PeerReStake = 7,
}
impl PeerAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PeerAction::InvalidAction => "InvalidAction",
            PeerAction::PeerRegistration => "PeerRegistration",
            PeerAction::PeerUnstaking => "PeerUnstaking",
            PeerAction::PeerDeregistration => "PeerDeregistration",
            PeerAction::PeerJailed => "PeerJailed",
            PeerAction::PeerUnJailed => "PeerUnJailed",
            PeerAction::PeerSlashed => "PeerSlashed",
            PeerAction::PeerReStake => "PeerReStake",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "InvalidAction" => Some(Self::InvalidAction),
            "PeerRegistration" => Some(Self::PeerRegistration),
            "PeerUnstaking" => Some(Self::PeerUnstaking),
            "PeerDeregistration" => Some(Self::PeerDeregistration),
            "PeerJailed" => Some(Self::PeerJailed),
            "PeerUnJailed" => Some(Self::PeerUnJailed),
            "PeerSlashed" => Some(Self::PeerSlashed),
            "PeerReStake" => Some(Self::PeerReStake),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Type {
    TxBlock = 0,
    StateBlock = 30,
    PeerBlock = 60,
    SmartContractResultBlock = 90,
    InvalidBlock = 120,
    ReceiptBlock = 150,
    RewardsBlock = 255,
}
impl Type {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Type::TxBlock => "TxBlock",
            Type::StateBlock => "StateBlock",
            Type::PeerBlock => "PeerBlock",
            Type::SmartContractResultBlock => "SmartContractResultBlock",
            Type::InvalidBlock => "InvalidBlock",
            Type::ReceiptBlock => "ReceiptBlock",
            Type::RewardsBlock => "RewardsBlock",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TxBlock" => Some(Self::TxBlock),
            "StateBlock" => Some(Self::StateBlock),
            "PeerBlock" => Some(Self::PeerBlock),
            "SmartContractResultBlock" => Some(Self::SmartContractResultBlock),
            "InvalidBlock" => Some(Self::InvalidBlock),
            "ReceiptBlock" => Some(Self::ReceiptBlock),
            "RewardsBlock" => Some(Self::RewardsBlock),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
