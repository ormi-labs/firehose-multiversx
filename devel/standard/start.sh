#!/usr/bin/env bash

ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

clean=
firemultiversx="$ROOT/../firemultiversx"

main() {
  pushd "$ROOT" &> /dev/null

  while getopts "hcfos" opt; do
    case $opt in
      h) usage && exit 0;;
      c) clean=true;;
      f) sync_connector && exit 0;;
      o) start_observing_squad && exit 0;;
      s) stop_observing_squad && exit 0;;
      \?) usage_error "Invalid option: -$OPTARG";;
    esac
  done
  shift $((OPTIND-1))
  [[ $1 = "--" ]] && shift

  set -e

  if [[ $clean == "true" ]]; then
    rm -rf firehose-data &> /dev/null || true
  fi

  exec "$firemultiversx" -c "$(basename "$ROOT")".yaml start "$@"
}

sync_connector() {
  # TODO: change to use latest tag
  local branch=0.0.3

  local dir_name=connector-repo

  git clone \
    https://github.com/multiversx/mx-chain-ws-connector-firehose-go ${dir_name} \
      --branch=${branch} \
      --single-branch \
      --depth=1

  pushd "${dir_name}/cmd/connector" &> /dev/null
  go build
  popd

  cp ${dir_name}/cmd/connector/connector ${ROOT} 
  cp -r ${dir_name}/cmd/connector/config ${ROOT}

  rm -rf ${dir_name}
}

start_observing_squad() {
    pushd "$ROOT/../observing-squad"
        bash ./run.sh setup && bash ./run.sh run
    popd
}

stop_observing_squad() {
    pushd "$ROOT/../observing-squad"
        bash ./run.sh cleanup
    popd

    if [[ $clean == "true" ]]; then
        rm -rf "$ROOT/OutportBlocks"
        sudo rm -rf "$ROOT/../observing-squad/MyObservingSquad"
    fi
}

usage_error() {
  message="$1"
  exit_code="$2"

  echo "ERROR: $message"
  echo ""
  usage
  exit "${exit_code:-1}"
}

usage() {
  echo "usage: start.sh [-c] [-f] [-o]"
  echo ""
  echo "Start $(basename "$ROOT") environment."
  echo ""
  echo "Options"
  echo "    -c             Clean actual data directory first"
  echo "    -f             Download and setup connector aggregator tool"
  echo "    -o             Setup and start observing squad"
  echo "    -s             Stop observing squad and removing containers"
}

main "$@"
