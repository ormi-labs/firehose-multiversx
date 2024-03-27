extern crate protoc_rust;

fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src/pb")
        .inputs(&["proto/sf/multiversx/type/v1/type.proto", "proto/guardians/guardians.proto", "/home/darius/dev/go/src/github.com/multiversx/mx-chain-core-go/data/outport/outportBlock.proto"])
        .include("proto")
        .include("/home/darius/dev/go/src/")
        .run()
        .expect("Running protoc failed.");
}

// extern crate prost_build;

// fn main() {
//     prost_build::compile_protos(
//         &[
//             "proto/sf/multiversx/type/v1/type.proto",
//             "proto/guardians/guardians.proto",
//             "/home/darius/dev/go/src/github.com/multiversx/mx-chain-core-go/data/outport/outportBlock.proto",
//         ],
//         &["proto", "/home/darius/dev/go/src/", "guardians"],
//     )
//     .unwrap();
// }
