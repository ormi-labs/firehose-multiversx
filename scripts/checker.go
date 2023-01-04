package main

import (
	"fmt"
	"io"
	"net/http"
	"os"

	"github.com/ElrondNetwork/elrond-go-core/hashing/blake2b"
	"github.com/ElrondNetwork/elrond-go-core/marshal"
	logger "github.com/ElrondNetwork/elrond-go-logger"
	"github.com/tidwall/gjson"
	"github.com/urfave/cli"
)

var log = logger.GetOrCreate("checker")
var marshaller = &marshal.GogoProtoMarshalizer{}
var hasher = blake2b.NewBlake2b()
var checkMeta = false

const proxyPem = "testnet/testnet-local/sandbox/proxy/config/walletKey.pem"
const esdtIssueAddress = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u"
const proxyUrl = "http://127.0.0.1:7950"

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

	txHash, err := sendIssueESDTTx(address, privateKey)
	if err != nil {
		return err
	}

	resp, err := http.Get(fmt.Sprintf("%s/transaction/%s?withResults=true", proxyUrl, txHash))
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
		return checkShardBlock(hyperBlockNonce, address.AddressAsBech32String(), txHash)
	}

	return checkMetaBlock(string(body), txHash)
}
