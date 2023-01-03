package main

import (
	"bytes"
	"encoding/hex"
	"fmt"
	"io"
	"math/big"
	"net/http"
	"os"

	core2 "github.com/ElrondNetwork/elrond-go-core/core"
	"github.com/ElrondNetwork/elrond-go-core/data/alteredAccount"
	"github.com/ElrondNetwork/elrond-go-core/data/firehose"
	"github.com/ElrondNetwork/elrond-go-core/data/transaction"
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

	scrs := gjson.Get(string(body), "data.transaction.smartContractResults").Array()
	logs := gjson.Get(string(body), "data.transaction.logs")

	blockNum := gjson.Get(string(body), "data.transaction.blockNonce").Uint()
	blockHash := gjson.Get(string(body), "data.transaction.blockHash").String()

	multiversxBlock, err := printOneBlockE(blockNum)
	if err != nil {
		return err
	}

	if !checkMeta {
		hyperBlockNonce := gjson.Get(string(body), "data.transaction.hyperblockNonce").Uint()
		return checkShardBlock(hyperBlockNonce, address)
	}

	err = checkHeader(multiversxBlock.MultiversxBlock, blockHash)
	if err != nil {
		return err
	}

	err = checkSCRs(scrs, multiversxBlock.MultiversxBlock.SmartContractResult)
	if err != nil {
		return err
	}

	err = checkLogs(logs, multiversxBlock.MultiversxBlock.Logs, gjson.Get(string(body), "data.transaction.hash").String())
	if err != nil {
		return err
	}

	err = checkAlteredAccounts(multiversxBlock.MultiversxBlock.AlteredAccounts)
	if err != nil {
		return err
	}

	return nil
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

	multiversxBlock, err := printOneBlockE(shardBlockNonce)
	if err != nil {
		return err
	}

	if hex.EncodeToString(multiversxBlock.MultiversxBlock.HeaderHash) != shardBlockHash {
		return fmt.Errorf("received invalid shard hash: %s, expected: %s",
			hex.EncodeToString(multiversxBlock.MultiversxBlock.HeaderHash),
			shardBlockHash)
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

func checkHeader(multiversxBlock *firehose.FirehoseBlock, expectedHash string) error {
	hashBytes := hasher.Compute(string(multiversxBlock.HeaderBytes))
	hashStr := hex.EncodeToString(hashBytes)

	if hashStr != expectedHash {
		return fmt.Errorf("invalid header hash, expected: %s, got: %s", expectedHash, hashStr)
	}

	return nil
}

func checkAlteredAccounts(alteredAccount []*alteredAccount.AlteredAccount) error {
	if len(alteredAccount) != 1 || alteredAccount[0].Address != "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u" {
		return fmt.Errorf("expected only one altered account: erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u, got: %d", len(alteredAccount))
	}

	balance, castOk := big.NewInt(0).SetString(alteredAccount[0].Balance, 10)
	if !castOk {
		return fmt.Errorf("could not convert balance: %s to bigInt", balance)
	}

	if balance.Cmp(big.NewInt(50000000000000000)) < 0 {
		return fmt.Errorf("expected balance increased after ESDT issue balance, but got %s", balance)
	}

	return nil
}

func checkSCRs(responses []gjson.Result, scrs map[string]*firehose.SCRWithFee) error {
	if len(responses) != len(scrs) {
		return fmt.Errorf("got %d scrs from api, but indexed %d scrs", len(responses), len(scrs))
	}

	for _, response := range responses {
		hash := response.Get("hash").String()
		hashBytes, err := hex.DecodeString(hash)
		if err != nil {
			return err
		}

		scrFromProtocol, found := scrs[string(hashBytes)]
		if !found {
			return fmt.Errorf("api hash %s not found in indexed block", hash)
		}

		computedHash, err := core2.CalculateHash(marshaller, hasher, scrFromProtocol.SmartContractResult)
		if err != nil {
			return err
		}

		if !bytes.Equal(computedHash, hashBytes) {
			return fmt.Errorf("computed scr hash != received scr hash")
		}
	}

	return nil
}

// TODO: CHECK HASH(HEADER)  =HASH

func checkLogs(responses gjson.Result, logs map[string]*transaction.Log, hash string) error {
	if len(logs) != 1 {
		return fmt.Errorf("expected only one generated log")
	}
	hashBytes, err := hex.DecodeString(hash)
	if err != nil {
		return err
	}

	indexedLog, found := logs[string(hashBytes)]
	if !found {
		return fmt.Errorf("hash %s not found in indexed logs", hash)
	}

	// TODO: BECH32 converter, check here address

	events := responses.Get("events").Array()
	if len(events) != len(indexedLog.Events) {
		return fmt.Errorf("got %d events from api, but indexed %d events", len(events), len(indexedLog.Events))
	}

	for _, event := range events {
		identifier := event.Get("identifier").String()
		if !contains(indexedLog.Events, identifier) {
			return fmt.Errorf("indexed event %s not found", identifier)
		}
	}

	return nil
}

func contains(events []*transaction.Event, identifier string) bool {
	for _, event := range events {
		if string(event.Identifier) == identifier {
			return true
		}
	}

	return false
}
