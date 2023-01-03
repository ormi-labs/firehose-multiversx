package main

import (
	"context"
	"time"

	"github.com/ElrondNetwork/elrond-sdk-erdgo/blockchain"
	"github.com/ElrondNetwork/elrond-sdk-erdgo/builders"
	"github.com/ElrondNetwork/elrond-sdk-erdgo/core"
	"github.com/ElrondNetwork/elrond-sdk-erdgo/interactors"
)

func sendIssueESDTTx(address core.AddressHandler, privateKey []byte) (string, error) {
	args := blockchain.ArgsElrondProxy{
		ProxyURL:            "http://127.0.0.1:7950",
		CacheExpirationTime: time.Minute,
		EntityType:          core.Proxy,
	}

	proxy, err := blockchain.NewElrondProxy(args)
	if err != nil {
		return "", err
	}

	txBuilder, err := builders.NewTxBuilder(blockchain.NewTxSigner())
	if err != nil {
		return "", err
	}

	ti, err := interactors.NewTransactionInteractor(proxy, txBuilder)
	if err != nil {
		return "", err
	}

	networkConfig, err := proxy.GetNetworkConfig(context.Background())
	if err != nil {
		return "", err
	}

	transactionArguments, err := proxy.GetDefaultTransactionArguments(context.Background(), address, networkConfig)
	if err != nil {
		return "", err
	}

	transactionArguments.Value = "50000000000000000"
	transactionArguments.GasLimit = 55141500
	transactionArguments.Data = []byte("issue@4141414141@41414141@6f@01@63616e55706772616465@74727565@63616e57697065@74727565@63616e467265657a65@74727565")
	transactionArguments.RcvAddr = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u"

	signedTx, err := ti.ApplySignatureAndGenerateTx(privateKey, transactionArguments)
	if err != nil {
		return "", err
	}

	hash, err := proxy.SendTransaction(context.Background(), signedTx)
	if err != nil {
		return "", err
	}
	log.Info("sent transaction", "tx hash", hash)

	log.Info("waiting 4 rounds...")
	time.Sleep(time.Millisecond * time.Duration(networkConfig.RoundDuration*4))

	return hash, nil
}
