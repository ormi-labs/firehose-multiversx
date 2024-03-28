// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutportBlock {
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
    #[prost(uint64, repeated, packed="false", tag="8")]
    pub signers_indexes: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, tag="9")]
    pub highest_final_block_nonce: u64,
    #[prost(bytes="vec", tag="10")]
    pub highest_final_block_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockData {
    #[prost(uint32, tag="1")]
    pub shard_id: u32,
    #[prost(bytes="vec", tag="2")]
    pub header_bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub header_type: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="4")]
    pub header_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="5")]
    pub body: ::core::option::Option<Body>,
    #[prost(message, repeated, tag="6")]
    pub intra_shard_mini_blocks: ::prost::alloc::vec::Vec<MiniBlock>,
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
    #[prost(uint64, repeated, packed="false", tag="2")]
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
