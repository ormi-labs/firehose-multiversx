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
ls

#sed -e "s/reader-node-path:.*/$CURRENT_DIR/" $CURRENT_DIR/../devel/standard/standard.yaml

#sed -i 's/reader-node-path:.*/reader-node-path: $CURRENT_DIR /' $CURRENT_DIR/../devel/standard/standard.yaml

#cd ../../devel/standard

#sudo ./start.sh
