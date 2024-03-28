GRAPH_CONFIG ?= ../graph-node-dev/config/graphman.toml
STOP_BLOCK ?= +1000

.PHONY: build protogen package test

build:
	cargo build --target wasm32-unknown-unknown --release

protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

package: build
	substreams pack -o substreams.spkg substreams.yaml

test:
	cargo test
