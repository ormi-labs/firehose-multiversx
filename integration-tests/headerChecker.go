package main

import (
	"encoding/hex"
	"fmt"

	mvxcore "github.com/multiversx/mx-chain-core-go/core"
	"github.com/multiversx/mx-chain-core-go/data/outport"
)

func checkHeader(
	multiversxBlock *outport.OutportBlock,
	expectedHash string,
	expectedHeaderTypes map[mvxcore.HeaderType]struct{},
) error {
	log.Info("checking header", "hash", expectedHash)

	hashBytes := hasher.Compute(string(multiversxBlock.BlockData.HeaderBytes))
	hashStr := hex.EncodeToString(hashBytes)
	if hashStr != expectedHash {
		return fmt.Errorf("checkHeader: invalid header hash after computation, expected: %s, got: %s", expectedHash, hashStr)
	}

	indexedHeaderHash := hex.EncodeToString(multiversxBlock.BlockData.HeaderHash)
	if indexedHeaderHash != expectedHash {
		return fmt.Errorf("checkHeader: indexed invalid header hash %s , expected %v", indexedHeaderHash, expectedHeaderTypes)
	}

	headerType := mvxcore.HeaderType(multiversxBlock.BlockData.HeaderType)
	_, found := expectedHeaderTypes[headerType]
	if !found {
		return fmt.Errorf("checkHeader: indexed invalid header type %s , expected %v", headerType, expectedHeaderTypes)
	}

	return nil
}
