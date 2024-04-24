.PHONY: build protogen package test

build:
	cargo build --target wasm32-unknown-unknown --release

protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

package: build
	substreams pack -o substreams.spkg substreams.yaml

test:
	cargo test

run: package
	substreams run substreams.spkg map_print_block --plaintext -e localhost:10016
