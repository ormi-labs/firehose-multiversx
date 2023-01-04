CURRENT_DIR=$(pwd)
SANDBOX_PATH=$CURRENT_DIR/testnet/testnet-local/sandbox
KEY_GENERATOR_PATH=$CURRENT_DIR/testnet/elrond-go/cmd/keygenerator

#./local-testnet.sh new

cd testnet/elrond-go/cmd/keygenerator
go build
./keygenerator
cd ../../../../

cd ../devel/standard/

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

./start.sh
