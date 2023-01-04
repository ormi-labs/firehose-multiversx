package main

import (
	"encoding/hex"
	"fmt"
	"io"
	"net/http"
	"os"

	core2 "github.com/ElrondNetwork/elrond-go-core/core"
	"github.com/ElrondNetwork/elrond-go-core/hashing/blake2b"
	"github.com/ElrondNetwork/elrond-go-core/marshal"
	logger "github.com/ElrondNetwork/elrond-go-logger"
	"github.com/ElrondNetwork/elrond-sdk-erdgo/core"
	"github.com/tidwall/gjson"
	"github.com/urfave/cli"
)

var log = logger.GetOrCreate("checker")
var marshaller = &marshal.GogoProtoMarshalizer{}
var hasher = blake2b.NewBlake2b()
var checkMeta = false

const proxyPem = "testnet/testnet-local/sandbox/proxy/config/walletKey.pem"
const esdtIssueAddress = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u"

func main() {
	app := cli.NewApp()
	app.Name = "Transaction sender tool"
	app.Usage = "This is the entry point for the tool that sends one tx/block from multiple shards"
	//app.Flags = getFlags()
	app.Authors = []cli.Author{
		{
			Name:  "The Elrond Team",
			Email: "contact@elrond.com",
		},
	}

	app.Action = func(c *cli.Context) error {
		return startProcess(c)
	}

	err := app.Run(os.Args)
	if err != nil {
		log.Error(err.Error())
		os.Exit(1)
		return
	}
}

func startProcess(c *cli.Context) error {
	address, privateKey, err := getAddressAndSK(proxyPem)
	if err != nil {
		return err
	}

	hash, err := sendIssueESDTTx(address, privateKey)
	if err != nil {
		return err
	}

	resp, err := http.Get(fmt.Sprintf("http://127.0.0.1:7950/transaction/%s?withResults=true", hash))
	if err != nil {
		return err
	}

	defer func() {
		err = resp.Body.Close()
		if err != nil {
			log.Warn("could not close response body", "error", err)
		}
	}()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return err
	}

	if !checkMeta {
		hyperBlockNonce := gjson.Get(string(body), "data.transaction.hyperblockNonce").Uint()
		return checkShardBlock(hyperBlockNonce, address)
	}

	return checkMetaBlock(string(body))
}

func checkShardBlock(hyperBlockNonce uint64, address core.AddressHandler) error {
	resp, err := http.Get(fmt.Sprintf("http://127.0.0.1:7950/hyperblock/by-nonce/%d", hyperBlockNonce))
	if err != nil {
		return err
	}
	defer resp.Body.Close()
	body, err := io.ReadAll(resp.Body)

	shardBlocks := gjson.Get(string(body), "data.hyperblock.shardBlocks").Array()

	if len(shardBlocks) != 1 {
		return fmt.Errorf("should only have one shard, but got %d", len(shardBlocks))
	}

	shardBlockHash := shardBlocks[0].Get("hash").String()
	shardBlockNonce := shardBlocks[0].Get("nonce").Uint()

	multiversxBlock, err := getBlockFromStorage(shardBlockNonce)
	if err != nil {
		return err
	}

	err = checkHeader(
		multiversxBlock.MultiversxBlock,
		shardBlockHash,
		map[core2.HeaderType]struct{}{
			core2.ShardHeaderV1: {},
			core2.ShardHeaderV2: {},
		})
	if err != nil {
		return err
	}

	apiTxs := gjson.Get(string(body), "data.hyperblock.transactions").Array()

	if len(apiTxs) < 1 || len(multiversxBlock.MultiversxBlock.Transactions) != 1 {
		return fmt.Errorf("expected only one sent tx, got %d", len(multiversxBlock.MultiversxBlock.Transactions))
	}

	// TODOD: CHECK HERE IT CONTAINS

	apiTxHash := apiTxs[0].Get("hash").String()

	txHashBytes, err := hex.DecodeString(apiTxHash)
	if err != nil {
		return err
	}

	protocolTx, found := multiversxBlock.MultiversxBlock.Transactions[string(txHashBytes)]
	if !found {
		return fmt.Errorf("expected indexed tx hash: %s not found", apiTxs)
	}

	txProtocolHash, err := core2.CalculateHash(marshaller, hasher, protocolTx.Transaction)
	if err != nil {
		return err
	}

	if hex.EncodeToString(txProtocolHash) != apiTxHash {
		return fmt.Errorf("invalid tx hash, expected: %s, got %s",
			hex.EncodeToString(txProtocolHash), apiTxHash)
	}

	if len(multiversxBlock.MultiversxBlock.AlteredAccounts) != 1 {
		return fmt.Errorf("got more altered accounts")
	}

	if !(multiversxBlock.MultiversxBlock.AlteredAccounts[0].Nonce > 0) {
		return fmt.Errorf("invalid nonce")
	}

	if multiversxBlock.MultiversxBlock.AlteredAccounts[0].Address != address.AddressAsBech32String() {
		return fmt.Errorf("invalid address")
	}

	return nil
}
