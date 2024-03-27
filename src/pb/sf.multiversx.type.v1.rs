// // @generated
// #[allow(clippy::derive_partial_eq_without_eq)]
// #[derive(Clone, PartialEq, ::prost::Message)]
// pub struct BlockHeader {
//     #[prost(uint64, tag="1")]
//     pub height: u64,
//     #[prost(string, tag="2")]
//     pub hash: ::prost::alloc::string::String,
//     #[prost(uint64, tag="3")]
//     pub previous_num: u64,
//     #[prost(string, tag="4")]
//     pub previous_hash: ::prost::alloc::string::String,
//     #[prost(uint64, tag="5")]
//     pub final_num: u64,
//     #[prost(string, tag="6")]
//     pub final_hash: ::prost::alloc::string::String,
//     #[prost(uint64, tag="7")]
//     pub timestamp: u64,
// }
// #[allow(clippy::derive_partial_eq_without_eq)]
// #[derive(Clone, PartialEq, ::prost::Message)]
// pub struct Block {
//     #[prost(message, optional, tag="1")]
//     pub header: ::core::option::Option<BlockHeader>,
//     #[prost(message, optional, tag="2")]
//     pub guardians: ::core::option::Option<super::super::super::super::guardians::Guardians>,
//     #[prost(message, optional, tag="3")]
//     pub blockk: ::core::option::Option<super::super::super::super::proto::OutportBlock>,
// }
// // @@protoc_insertion_point(module)
