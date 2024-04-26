#!/bin/bash
# Copyright 2021 dfuse Platform Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && cd .. && pwd )"

# Protobuf definitions
PROTO_MULTIVERSX=${2:-"$ROOT/proto"}

function main() {
  checks

  set -e
  cd "$ROOT/pb" &> /dev/null

  sync_data_proto_file "0.0.1"

  generate "sf/multiversx/type/v1/type.proto"

  echo "generate.sh - `LANG=en_US date --utc` - `whoami`" > ./last_generate.txt
  echo "multiversx/firehose-multiversx/proto revision: `GIT_DIR=$ROOT/.git git log -n 1 --pretty=format:%h -- proto`" >> ./last_generate.txt
}

function sync_data_proto_file() {
    branch="main"
    if [[ "$#" -gt 0 ]]; then
      branch="$1"; shift
    fi

    proto_file_path=$ROOT/proto/sf/multiversx/type/v1/type.proto

    if [ -f ${proto_file_path} ]; then
        mv ${proto_file_path} ${proto_file_path}.tmp
    fi

    curl -sL \
        -o ${proto_file_path} \
        https://raw.githubusercontent.com/multiversx/mx-chain-ws-connector-firehose-go/${branch}/data/hyperOutportBlocks/hyperOutportBlock.proto

    # restore old proto file on error
    if [[ "$?" -ne 0 ]]; then
        mv ${proto_file_path}.tmp ${proto_file_path}
        return
    fi

    # update proto file
    new_package_name="sf.multiversx.type.v1"
    sed -i -r "s/^package\s(.*);/package ${new_package_name};/" ${proto_file_path}
    if [[ "$?" -ne 0 ]]; then
        mv ${proto_file_path}.tmp ${proto_file_path}
        return
    fi

    new_go_package_name="github.com/multiversx/firehose-multiversx/pb/sf/multiversx/type/v1;pbmultiversx"
    escaped_go_package_name=$(printf '%s\n' "${new_go_package_name}" | sed -e 's/[\/&]/\\&/g')
    sed -i -r "s/^option go_package\s=\s\"(.*)\";$/option go_package = \"${escaped_go_package_name}\";/" ${proto_file_path}
    if [[ "$?" -ne 0 ]]; then
        mv ${proto_file_path}.tmp ${proto_file_path}
        return
    fi
}

# usage:
# - generate <protoPath>
# - generate <protoBasePath/> [<file.proto> ...]
function generate() {
    base=""
    if [[ "$#" -gt 1 ]]; then
      base="$1"; shift
    fi

    for file in "$@"; do
      protoc -I$GOPATH/src -I$PROTO_MULTIVERSX \
        --go_out=. --go_opt=paths=source_relative \
         $base$file
    done
}

function checks() {
  # The old `protoc-gen-go` did not accept any flags. Just using `protoc-gen-go --version` in this
  # version waits forever. So we pipe some wrong input to make it exit fast. This in the new version
  # which supports `--version` correctly print the version anyway and discard the standard input
  # so it's good with both version.
  result=$(printf "" | protoc-gen-go --version 2>&1 | grep -Eo "v[0-9\.]+")
  if [[ "$result" == "" ]]; then
    echo "Your version of 'protoc-gen-go' (at `which protoc-gen-go`) is not recent enough."
    echo ""
    echo "To fix your problem, perform those commands:"
    echo ""
    echo "  go install google.golang.org/protobuf/cmd/protoc-gen-go@v1.25.0"
    echo "  go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@v1.1.0"
    echo ""
    echo "If everything is working as expected, the command:"
    echo ""
    echo "  protoc-gen-go --version"
    echo ""
    echo "Should print 'protoc-gen-go v1.25.0' (if it just hangs, you don't have the correct version)"
    exit 1
  fi
}

main "$@"
