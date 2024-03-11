CURRENT_DIR=$(pwd)
FIREHOSE_CONNECTOR_PATH=$CURRENT_DIR/testnet/mx-chain-ws-connector-firehose-go/cmd/connector

cd ../../devel/standard/ || exit
DEVEL=$(pwd)

rm -rf firehose-data/*
sed -i "s@reader-node-path:.*@reader-node-path: \"$DEVEL/connector\"@" "$DEVEL"/standard.yaml

cp "$FIREHOSE_CONNECTOR_PATH"/connector "$DEVEL"
cp -R  "$FIREHOSE_CONNECTOR_PATH"/config "$DEVEL"

./start.sh -c
