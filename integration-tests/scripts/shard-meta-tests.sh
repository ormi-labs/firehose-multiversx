#!/usr/bin/env bash

cd ..
go build
cd scripts

startTest(){
  echo "starting firehosenode with screen"

  screen -L -A -m -d -S firehosenode ./firehose-node.sh $1

  sleep 50

  echo "finish starting firehosenode"
  echo "starting integration tests for $1"

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

  echo "finished integration tests for $1"

  screen -S firehosenode -X quit
  cd scripts
}

startTest shard || exit 1
startTest metachain || exit 1
