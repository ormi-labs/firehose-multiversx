#!/usr/bin/env bash

# set -x

SCRIPTPATH="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

declare -a NODES=("0" "1" "2" "metachain")

STACK_FOLDER=${SCRIPTPATH}/MyObservingSquad
KEYS_FOLDER=${STACK_FOLDER}/keys
EXTERNAL_CONFIG=${SCRIPTPATH}/external.toml

setup() {
    docker pull multiversx/chain-keygenerator:latest

    gen_dirs

    gen_keys
}

gen_dirs() {
    mkdir -p ${KEYS_FOLDER}

    for id in "${NODES[@]}"
    do
        mkdir -p ${STACK_FOLDER}/node-${id}/{config,logs}
        mkdir -p ${STACK_FOLDER}/node-${id}/db
    done
}

gen_keys() {
    for id in "${NODES[@]}"
    do
        docker run \
           --rm \
           --mount type=bind,source=${KEYS_FOLDER},destination=/keys \
           --workdir /keys \
           multiversx/chain-keygenerator:latest && \
           sudo chown $(whoami) ${KEYS_FOLDER}/validatorKey.pem && \
           mv ${KEYS_FOLDER}/validatorKey.pem ${STACK_FOLDER}/node-$id/config/observerKey_$id.pem

    done
}

sync_external_file() {
    curl -sL \
        -o ${proto_file_path} \
        https://raw.githubusercontent.com/multiversx/mx-chain-mainnet-config/master/external.toml

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
}

run_observers() {
    for id in "${NODES[@]}"
    do
        cp ${SCRIPTPATH}/external.toml ${STACK_FOLDER}/node-$id/config/external.toml
    done

    let i=0

    for shard_id in "${NODES[@]}"
    do
        observer_dir=${STACK_FOLDER}/node-${shard_id}
        p2p_port=$(( 10000 + $i + 3 ))
        http_port=$(( 8080 + $i + 1 ))

        docker run \
            -d \
            --mount type=bind,source=${observer_dir}/db,destination=/go/mx-chain-go/cmd/node/db \
            --mount type=bind,source=${observer_dir}/logs,destination=/go/mx-chain-go/cmd/node/logs \
            --mount type=bind,source=${observer_dir}/config,destination=/config \
            --network="host" \
            --name squad-${shard_id} \
            multiversx/chain-testnet:T1.7.11.0 \
            --destination-shard-as-observer=${shard_id} \
            --validator-key-pem-file=/config/observerKey_${shard_id}.pem \
            --display-name="${display_name}" \
            --config-external=/config/external.toml \
            --rest-api-interface=localhost:${http_port} \
            # --full-archive
        if [ "$?" -ne 0 ]; then
            echo -e "failed to start container"
            exit 1
        fi

        echo -e "created container: dir path = ${observer_dir}"

        i=$(( i+1 ))
    done
}

run_stack() {
    run_observers
}

start_stack() {
    for shard_id in "${NODES[@]}"
    do
        docker start squad-${shard_id}
        echo -e "started container: shard_id = ${shard_id}"
    done
}

stop_stack() {
    for shard_id in "${NODES[@]}"
    do
        docker stop squad-${shard_id}
        echo -e "stopping container: shard_id = ${shard_id}"
    done
}

rm_stack() {
    for shard_id in "${NODES[@]}"
    do
        docker rm squad-${shard_id}
        echo -e "removing container: shard_id = ${shard_id}"
    done
}

main() {
    case $1 in
        "setup") 
            setup
            ;;
        "run") 
            run_stack
            ;;
        "start") 
            start_stack
            ;;
        "stop")
            stop_stack
            ;;
        "cleanup")
            stop_stack
            rm_stack
            ;;
        *)
            echo
            ;;
    esac
}

main $@
