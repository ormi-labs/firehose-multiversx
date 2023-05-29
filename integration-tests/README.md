## About

This folder contains integration tests scripts to start firehose ingestion nodes for metachain and shard(0) with a local
testnet and check indexed data validity for custom a scenario(`ESDTIssue`).

### Data flow

This chapter illustrates how MultiversX handles exported data using WebSocket Exporter Outport Driver.

Consider a network consisting of nodes in a sharded architecture, all being orchestrated by a metachain. Adding an
observer (in either shard), and making it act as a WebSocket Exporter(by enabling a config file), it will extract and
export data. This exported data is then ingested by the Firehose Connector, which processes it through the StreamingFast
framework, enabling efficient data streaming and analysis.

```
+-----------------------+       +------------------------+       +-----------------------+
|    Local Testnet      | ----> |   WebSocket Exporter   | ----> |   Firehose Connector  |
+-----------------------+       +------------------------+       +-----------------------+
| +-------+ +---------+ |       |   +----------------+   |       |   +----------------+  |
| |Shard1 | |  Shard2 | |       |   | Observer in    |   |       |   | StreamingFast  |  |
| +-------+ +---------+ |       |   | Shard x        |   |       |   | Data Processing|  |
| |Shard3 | |Metachain| |       |   +----------------+   |       |   +----------------+  |
| +-------+ +---------+ |       |                        |       +-----------------------+
+-----------------------+       +------------------------+
```

### Scripts:

- `local-testnet.sh` - used to create/start/reset/stop a local MultiversX testnet
- `exporter-node.sh` - setup for a new observer node which will be enabled to export data. Each observer node has to
  be specified in which shard to start (shard0/meta). This node will export real-time data to the firehose connector
- `firehose-connector.sh` - setup for a new firehose connector, which will connect to the exporter node to receive real
  time data. This is opened within the streaming-fast indexing processing node.
- `shard-meta-tests.sh` - starts with screen a new exporter node and a firehose connector in metachain+shard0 and tests
  the custom scenario, by building(`main.go`) and executing (`checker`) binary.

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
./local-testnet.sh stop
```

If you want to locally run a firehose node without integration tests, you need to:

- start a local testnet, as previously described at step 1
- start your observer node either in shard in metachain to enable exporting data
- start the firehose connector node to receive incoming data

```bash
./exporter-node.sh shard # starts a firehose node in shard 0. Otherwise, you can call the script with metachain parameter
./firehose-connector.sh
```