package main

import (
	"bytes"
	"encoding/hex"
	"fmt"
	"math/big"

	core2 "github.com/ElrondNetwork/elrond-go-core/core"
	"github.com/ElrondNetwork/elrond-go-core/data/alteredAccount"
	"github.com/ElrondNetwork/elrond-go-core/data/firehose"
	"github.com/ElrondNetwork/elrond-go-core/data/transaction"
	"github.com/tidwall/gjson"
)

func checkMetaBlock(apiTxResultBody string) error {
	blockNum := gjson.Get(apiTxResultBody, "data.transaction.blockNonce").Uint()
	multiversxBlock, err := getBlockFromStorage(blockNum)
	if err != nil {
		return err
	}

	blockHash := gjson.Get(apiTxResultBody, "data.transaction.blockHash").String()
	err = checkHeader(
		multiversxBlock.MultiversxBlock,
		blockHash,
		map[core2.HeaderType]struct{}{
			core2.MetaHeader: {},
		})
	if err != nil {
		return err
	}

	scrs := gjson.Get(apiTxResultBody, "data.transaction.smartContractResults").Array()
	err = checkMetaSCRs(scrs, multiversxBlock.MultiversxBlock.SmartContractResult)
	if err != nil {
		return err
	}

	logs := gjson.Get(apiTxResultBody, "data.transaction.logs")
	txHash := gjson.Get(apiTxResultBody, "data.transaction.hash").String()
	err = checkLogs(logs, multiversxBlock.MultiversxBlock.Logs, txHash)
	if err != nil {
		return err
	}

	err = checkMetaAlteredAccounts(multiversxBlock.MultiversxBlock.AlteredAccounts)
	if err != nil {
		return err
	}

	return nil
}

func checkHeader(
	multiversxBlock *firehose.FirehoseBlock,
	expectedHash string,
	expectedHeaderTypes map[core2.HeaderType]struct{},
) error {
	hashBytes := hasher.Compute(string(multiversxBlock.HeaderBytes))
	hashStr := hex.EncodeToString(hashBytes)
	if hashStr != expectedHash {
		return fmt.Errorf("checkHeader: invalid header hash after computation, expected: %s, got: %s", expectedHash, hashStr)
	}

	indexedHeaderHash := hex.EncodeToString(multiversxBlock.HeaderHash)
	if indexedHeaderHash != expectedHash {
		return fmt.Errorf("checkHeader: indexed invalid header hash %s , expected %v", indexedHeaderHash, expectedHeaderTypes)
	}

	headerType := core2.HeaderType(multiversxBlock.HeaderType)
	_, found := expectedHeaderTypes[headerType]
	if !found {
		return fmt.Errorf("checkHeader: indexed invalid header type %s , expected %v", headerType, expectedHeaderTypes)
	}

	return nil
}

func checkMetaAlteredAccounts(alteredAccount []*alteredAccount.AlteredAccount) error {
	if len(alteredAccount) != 1 || alteredAccount[0].Address != esdtIssueAddress {
		return fmt.Errorf("checkMetaAlteredAccounts: expected only one altered account: %s, got: %d", esdtIssueAddress, len(alteredAccount))
	}

	balance, castOk := big.NewInt(0).SetString(alteredAccount[0].Balance, 10)
	if !castOk {
		return fmt.Errorf("checkMetaAlteredAccounts: could not convert balance: %s to bigInt, address: %s", balance, esdtIssueAddress)
	}

	if balance.Cmp(big.NewInt(50000000000000000)) < 0 {
		return fmt.Errorf("checkMetaAlteredAccounts: expected %s address' balance increased after ESDT issue balance, but got %s", esdtIssueAddress, balance)
	}

	return nil
}

func checkMetaSCRs(apiSCRs []gjson.Result, scrs map[string]*firehose.SCRWithFee) error {
	numApiSCRS := len(apiSCRs)
	numIndexedSCRS := len(scrs)
	if numApiSCRS != numIndexedSCRS {
		return fmt.Errorf("checkMetaSCRs: got %d scrs from api, but indexed %d scrs", numApiSCRS, numIndexedSCRS)
	}

	for _, apiSCR := range apiSCRs {
		hash := apiSCR.Get("hash").String()
		hashBytes, err := hex.DecodeString(hash)
		if err != nil {
			return err
		}

		scrFromProtocol, found := scrs[string(hashBytes)]
		if !found {
			return fmt.Errorf("checkMetaSCRs: api hash %s not found in indexed block", hash)
		}

		computedHash, err := core2.CalculateHash(marshaller, hasher, scrFromProtocol.SmartContractResult)
		if err != nil {
			return err
		}

		if !bytes.Equal(computedHash, hashBytes) {
			return fmt.Errorf("checkMetaSCRs: computed scr hash: %s != received scr hash: %s",
				hex.EncodeToString(computedHash), hash)
		}
	}

	return nil
}

func checkLogs(apiLog gjson.Result, logs map[string]*transaction.Log, txHash string) error {
	numIndexedLogs := len(logs)
	if numIndexedLogs != 1 {
		return fmt.Errorf("checkLogs: expected only one generated and indexed log, received %d", numIndexedLogs)
	}
	txHashBytes, err := hex.DecodeString(txHash)
	if err != nil {
		return err
	}

	indexedLog, found := logs[string(txHashBytes)]
	if !found {
		return fmt.Errorf("checkLogs: api tx hash %s not found in indexed logs", txHash)
	}

	apiEvents := apiLog.Get("events").Array()
	indexedEvents := indexedLog.Events
	numApiEvents := len(apiEvents)
	numIndexedAEvents := len(indexedEvents)
	if numApiEvents != numIndexedAEvents {
		return fmt.Errorf("checkLogs: got %d events from api, but indexed %d events", numApiEvents, numIndexedAEvents)
	}

	for _, apiEvent := range apiEvents {
		identifier := apiEvent.Get("identifier").String()
		if !contains(indexedEvents, identifier) {
			return fmt.Errorf("checkLogs: indexed event identifier %s not found", identifier)
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
