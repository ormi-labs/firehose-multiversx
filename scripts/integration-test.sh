CURRENT_DIR=$(pwd)
SANDBOX_PATH=$CURRENT_DIR/testnet/testnet-local/sandbox
KEY_GENERATOR_PATH=$CURRENT_DIR/testnet/elrond-go/cmd/keygenerator

go build

cd testnet/elrond-go/cmd/keygenerator
go build
./keygenerator
cd ../../../../../devel/standard/

rm -rf compiledSCStorage/
rm -rf config/
rm -rf db/
rm -rf stats/
rm -rf firehose-data/*
rm -rf logs/
rm node

mkdir "config/"

DEVEL=$(pwd)
cp $SANDBOX_PATH/node/node $DEVEL
cp -R $SANDBOX_PATH/node/config $DEVEL
mv config/config_observer.toml config/config.toml
mv $KEY_GENERATOR_PATH/validatorKey.pem config/

sed -i 's/DestinationShardAsObserver =.*/DestinationShardAsObserver = "0"/' $DEVEL/config/prefs.toml
sed -i 's/FullArchive =.*/FullArchive = true/' $DEVEL/config/prefs.toml
sed -i "s@reader-node-path:.*@reader-node-path: \"$DEVEL/node\"@" $DEVEL/standard.yaml



echo "starting firehosenode with screen"

screen -L -A -m -d -S firehosenode ./start.sh

sleep 55

echo "finish starting firehosenode"
cd ../../scripts

echo "integration tests started for shard"
./scripts

echo "finished integration tests for shard"

screen -S firehosenode -X quit
