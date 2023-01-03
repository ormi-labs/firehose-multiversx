package main

import (
	"bytes"
	"context"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"math/big"
	"net/http"
	"os"
	"time"

	core2 "github.com/ElrondNetwork/elrond-go-core/core"
	"github.com/ElrondNetwork/elrond-go-core/data/alteredAccount"
	"github.com/ElrondNetwork/elrond-go-core/data/firehose"
	"github.com/ElrondNetwork/elrond-go-core/data/transaction"
	"github.com/ElrondNetwork/elrond-go-core/hashing/blake2b"
	"github.com/ElrondNetwork/elrond-go-core/marshal"
	logger "github.com/ElrondNetwork/elrond-go-logger"
	"github.com/ElrondNetwork/elrond-sdk-erdgo/blockchain"
	"github.com/ElrondNetwork/elrond-sdk-erdgo/builders"
	"github.com/ElrondNetwork/elrond-sdk-erdgo/core"
	"github.com/ElrondNetwork/elrond-sdk-erdgo/interactors"
	pbmultiversx "github.com/ElrondNetwork/firehose-multiversx/types/pb/sf/multiversx/type/v1"
	"github.com/streamingfast/bstream"
	"github.com/streamingfast/dstore"
	pbbstream "github.com/streamingfast/pbgo/sf/bstream/v1"
	"github.com/tidwall/gjson"
	"github.com/urfave/cli"
	"google.golang.org/protobuf/proto"
)

var log = logger.GetOrCreate("checker")
var marshaller = &marshal.GogoProtoMarshalizer{}
var hasher = blake2b.NewBlake2b()

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

	//return nil
	args := blockchain.ArgsElrondProxy{
		ProxyURL:            "http://127.0.0.1:7950",
		CacheExpirationTime: time.Minute,
		EntityType:          core.Proxy,
	}

	proxy, err := blockchain.NewElrondProxy(args)
	if err != nil {
		return err
	}

	//cfg, err := proxy.GetNetworkConfig(context.Background())
	//if err != nil {
	//	return err
	//}

	w := interactors.NewWallet()
	privateKey, err := w.LoadPrivateKeyFromPemFile("testnet/testnet-local/sandbox/proxy/config/walletKey.pem")
	if err != nil {
		log.Error("unable to load alice.pem", "error", err)
		return err
	}
	// Generate address from private key
	address, err := w.GetAddressFromPrivateKey(privateKey)
	if err != nil {
		log.Error("unable to load the address from the private key", "error", err)
		return err
	}
	fmt.Println(address)

	txBuilder, err := builders.NewTxBuilder(blockchain.NewTxSigner())
	if err != nil {
		return err
	}

	ti, err := interactors.NewTransactionInteractor(proxy, txBuilder)
	if err != nil {
		return err
	}

	networkConfig, err := proxy.GetNetworkConfig(context.Background())
	if err != nil {
		return err
	}

	transactionArguments, err := proxy.GetDefaultTransactionArguments(context.Background(), address, networkConfig)
	if err != nil {
		return err
	}

	transactionArguments.Value = "50000000000000000"
	transactionArguments.GasLimit = 55141500
	transactionArguments.Data = []byte("issue@4141414141@41414141@6f@01@63616e55706772616465@74727565@63616e57697065@74727565@63616e467265657a65@74727565")
	transactionArguments.RcvAddr = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u"

	/*tx := &data.Transaction{
		Nonce:     acc.Nonce,
		Value:     "5000000000000000000",
		RcvAddr:   "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
		SndAddr:   "",
		GasPrice:  transactionArguments.GasPrice,
		GasLimit:  55141500,
		Data:      []byte("issue@4141414141@41414141@6f@01@63616e55706772616465@74727565@63616e57697065@74727565@63616e467265657a65@74727565"),
		Signature: "",
		ChainID:   "local-testnet",
	}

	*/

	signedTx, err := ti.ApplySignatureAndGenerateTx(privateKey, transactionArguments)
	if err != nil {
		return err
	}

	hash, err := proxy.SendTransaction(context.Background(), signedTx)
	if err != nil {
		return err
	}

	log.Info("sent transaction", "tx hash", hash)

	resp, err := http.Get(fmt.Sprintf("http://127.0.0.1:7950/transaction/%s?withResults=true", hash))
	if err != nil {
		// handle error
	}
	defer resp.Body.Close()
	body, err := io.ReadAll(resp.Body)

	fmt.Println(string(body))
	scrs := gjson.Get(string(body), "data.transaction.smartContractResults").Array()
	logs := gjson.Get(string(body), "data.transaction.logs")

	blockNum := gjson.Get(string(body), "data.transaction.blockNonce").Uint()
	blockHash := gjson.Get(string(body), "data.transaction.blockHash").String()

	multiversxBlock, err := printOneBlockE(blockNum)
	if err != nil {
		return err
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
	//printOneBlockE(3)

	return nil

	_ = proxy
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
	if len(alteredAccount) != 1 && alteredAccount[0].Address != "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u" {
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

func blockReaderFactory(reader io.Reader) (bstream.BlockReader, error) {
	return bstream.NewDBinBlockReader(reader, func(contentType string, version int32) error {
		protocol := pbbstream.Protocol(pbbstream.Protocol_value[contentType])
		if protocol != pbbstream.Protocol_ETH && version != 1 {
			return fmt.Errorf("reader only knows about %s block kind at version 1, got %s at version %d", protocol, contentType, version)
		}

		return nil
	})
}

func BlockDecoder(blk *bstream.Block) (interface{}, error) {
	if blk.Kind() != pbbstream.Protocol_UNKNOWN {
		return nil, fmt.Errorf("expected kind %s, got %s", pbbstream.Protocol_UNKNOWN, blk.Kind())
	}

	if blk.Version() != 1 {
		return nil, fmt.Errorf("this decoder only knows about version 1, got %d", blk.Version())
	}

	block := new(pbmultiversx.Block)
	payload, err := blk.Payload.Get()
	if err != nil {
		return nil, fmt.Errorf("getting payload: %w", err)
	}

	err = proto.Unmarshal(payload, block)
	if err != nil {
		return nil, fmt.Errorf("unable to decode payload: %w", err)
	}

	return block, nil
}

func printOneBlockE(blockNum uint64) (*pbmultiversx.Block, error) {
	bstream.GetBlockReaderFactory = bstream.BlockReaderFactoryFunc(blockReaderFactory)
	bstream.GetBlockDecoder = bstream.BlockDecoderFunc(BlockDecoder)
	bstream.GetBlockWriterHeaderLen = 10
	bstream.GetBlockPayloadSetter = bstream.MemoryBlockPayloadSetter
	bstream.GetMemoizeMaxAge = 20 * time.Second
	str := "../devel/standard/firehose-data/storage/one-blocks"

	store, err := dstore.NewDBinStore(str)
	if err != nil {
		return nil, fmt.Errorf("unable to create store at path %q: %w", store, err)
	}

	ctx := context.Background()
	var files []string
	filePrefix := fmt.Sprintf("%010d", blockNum)
	err = store.Walk(ctx, filePrefix, func(filename string) (err error) {
		files = append(files, filename)
		return nil
	})
	if err != nil {
		return nil, fmt.Errorf("unable to find on block files: %w", err)
	}

	var nativeBlock *pbmultiversx.Block
	for _, filepath := range files {
		ppp := str + "/" + filepath + ".dbin.zst"

		_ = ppp
		reader, err := store.OpenObject(ctx, filepath)
		if err != nil {
			fmt.Printf("❌ Unable to read block filename %s: %s\n", ppp, err)
			return nil, err
		}
		defer reader.Close()

		readerFactory, err := bstream.GetBlockReaderFactory.New(reader)
		if err != nil {
			fmt.Printf("❌ Unable to read blocks filename %s: %s\n", filepath, err)
			return nil, err
		}

		//fmt.Printf("One Block File: %s\n", store.ObjectURL(filepath))

		block, err := readerFactory.Read()
		if err != nil {
			if err == io.EOF {
				break
			}
			return nil, fmt.Errorf("reading block: %w", err)
		}

		nativeBlock, err = printBlock(block)
		if err != nil {
			return nil, err
		}

	}
	return nativeBlock, nil
}

func printBlock(block *bstream.Block) (*pbmultiversx.Block, error) {
	nativeBlock := block.ToProtocol().(*pbmultiversx.Block)

	data, err := json.MarshalIndent(nativeBlock, "", "  ")
	if err != nil {
		return nil, fmt.Errorf("json marshall: %w", err)
	}

	fmt.Println(string(data))

	return nativeBlock, nil
}
