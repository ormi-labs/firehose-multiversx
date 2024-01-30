#!/usr/bin/env bash

CURRENT_DIR=$(pwd)
SANDBOX_PATH=$CURRENT_DIR/testnet/testnet-local/sandbox
KEY_GENERATOR_PATH=$CURRENT_DIR/testnet/mx-chain-go/cmd/keygenerator

OBSERVER_MODE="server"

setup(){
  echo "starting integration tests for shard $1"

  pushd "$CURRENT_DIR" || exit

  cd testnet/mx-chain-go/cmd/keygenerator || exit
  go build
  ./keygenerator

  popd || exit

  rm -rf exporterNode/

  mkdir "exporterNode"
  cd exporterNode || exit

  rm -rf compiledSCStorage/
  rm -rf config/
  rm -rf db/
  rm -rf stats/
  rm -rf logs/
  rm node

  mkdir "config/"

  DEVEL=$(pwd)
  cp "$SANDBOX_PATH"/node/node "$DEVEL"
  cp -R "$SANDBOX_PATH"/node/config "$DEVEL"
  mv config/config_observer.toml config/config.toml
  mv "$KEY_GENERATOR_PATH"/validatorKey.pem config/

  sed -i "s@DestinationShardAsObserver =.*@DestinationShardAsObserver = \"$1\"@" "$DEVEL"/config/prefs.toml
  sed -i 's/FullArchive =.*/FullArchive = true/' "$DEVEL"/config/prefs.toml

  sed -i '/HostDriverConfig\]/!b;n;n;c\    Enabled = true' "$DEVEL"/config/external.toml
  sed -i "s@Mode =.*@Mode = \"$OBSERVER_MODE\"@" "$DEVEL"/config/external.toml
  sed -i 's/MarshallerType =.*/MarshallerType = "gogo protobuf"/' "$DEVEL"/config/external.toml
  sed -i 's/BlockingAckOnError =.*/BlockingAckOnError = false/' "$DEVEL"/config/external.toml

  ./node --log-level *:INFO
}

echoOptions(){
  echo "This script will start a firehose node in the provided shard(metachain/shard).
  The only acceptable parameters are shard(will use shard 0) and metachain for the current test configuration."
}

main(){
    if [ $# -eq 1 ]; then
      case "$1" in
        metachain)
          setup metachain;;
        shard)
          setup 0;;
        *)
          echoOptions;;
        esac
    else
      echoOptions
    fi
}

main "$@"
