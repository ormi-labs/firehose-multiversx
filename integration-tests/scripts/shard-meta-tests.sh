#!/usr/bin/env bash

cd ..
go build
cd scripts || exit

startExporterNode(){
    echo "###### starting exporter node with screen ######"

    mkdir -p screenLogs &&\
    touch screenLogs/exporter-node.log &&\
    screen -L -A -m -d -S exporternode bash -c\
     './exporter-node.sh "$1" >> screenLogs/exporter-node.log 2>&1' -- "$1"

    sleep 45

    echo "###### finished starting exporter node ######"
}

startFirehoseConnector(){
    echo "###### starting firehose connector with screen ######"

    sudo chmod +x firehose-connector.sh

    touch screenLogs/firehose-connector.log &&\
    screen -L -A -m -d -S connector bash -c\
    './firehose-connector.sh >> screenLogs/firehose-connector.log 2>&1'

    sleep 20

    echo "###### finished starting firehose connector ######"
}

startTest(){
  startExporterNode $1
  startFirehoseConnector

  echo "###### starting integration tests for $1 ######"

  cd ..
  if [[ "$1" == "shard" ]]
  then
    ./checker
    if [[ $? -ne 0 ]]; then
      exit 1
    fi
  else
    ./checker --check-meta
    if [[ $? -ne 0 ]]; then
      exit 1
    fi
  fi

  echo "###### finished integration tests for $1 ######"

  screen -S exporternode -X quit
  screen -S connector -X quit

  cd scripts || exit
}

startTest shard || exit 1
startTest metachain || exit 1
