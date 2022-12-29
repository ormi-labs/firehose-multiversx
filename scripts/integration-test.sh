#source  $(dirname "$0")/local-testnet.sh

CURRENT_DIR=$(pwd)
SANDBOX_PATH=$CURRENT_DIR/testnet/testnet-local/sandbox

#./local-testnet.sh new

cp $SANDBOX_PATH/node/node $CURRENT_DIR
cp -R $SANDBOX_PATH/node/config $CURRENT_DIR
mv config/config_observer.toml config/config.toml

sed -i 's/DestinationShardAsObserver =.*/DestinationShardAsObserver = "metachain"/' $CURRENT_DIR/config/prefs.toml
sed -i 's/FullArchive =.*/FullArchive = true/' $CURRENT_DIR/config/prefs.toml

cd ../devel/standard/

rm -rf compiledSCStorage/
rm -rf config/
rm -rf db/
rm -rf stats/
rm -rf firehose-data/*
rm -rf logs/
rm node

echo $(pwd)

mkdir "config/"

DEVEL=$(pwd)
cp $CURRENT_DIR/node $DEVEL
cp -R $CURRENT_DIR/config $DEVEL
cp $DEVEL/validatorKey.pem config/

ls

#sed -e "s/reader-node-path:.*/$CURRENT_DIR/" $CURRENT_DIR/../devel/standard/standard.yaml

#sed -i 's/reader-node-path:.*/reader-node-path: $CURRENT_DIR /' $CURRENT_DIR/../devel/standard/standard.yaml

#cd ../../devel/standard

#sudo ./start.sh
