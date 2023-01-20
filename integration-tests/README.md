## About

This folder contains integration tests scripts to start firehose ingestion nodes for metachain and shard(0) with a local
testnet and check indexed data validity for custom a scenario(`ESDTIssue`).

### Scripts:

- `local-testnet.sh` - used to create/start/reset/stop a local MultiversX testnet
- `firehose-node.sh` - setup for a new firehose node to start indexing process node. This firehose node will internally
  start a multiversx observer node (shard0/meta)
- `shard-meta-tests.sh` - starts with screen a new firehose node in metachain+shard0 and tests the custom scenario, by
  building(`main.go`) and executing (`checker`) binary.

- `main.go` contains the main scenario. It will:

1. Load an address with balance from the local testnet setup
2. Send an ESDT issue tx and get its hash
3. Depending on input flag parameter (meta/shard), it will check the block in which the hash was included and start
   validity data checks. For shard, it has to do an additional search in the hyperBlock to find in which block the
   transaction was included.

## How to use

1. Create and start a local testnet:

```bash
cd integration-tests/scripts
./local-testnet.sh new
./local-testnet.sh start
```

2. Run the integration tests:

```bash
./shard-meta-tests.sh
```

3. Stop the local testnet:

```bash
./shard-meta-tests.sh stop
```
