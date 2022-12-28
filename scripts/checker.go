package main

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"time"

	logger "github.com/ElrondNetwork/elrond-go-logger"
	"github.com/ElrondNetwork/elrond-sdk-erdgo/blockchain"
	"github.com/ElrondNetwork/elrond-sdk-erdgo/builders"
	"github.com/ElrondNetwork/elrond-sdk-erdgo/core"
	"github.com/ElrondNetwork/elrond-sdk-erdgo/interactors"
	pbmultiversx "github.com/ElrondNetwork/firehose-multiversx/types/pb/sf/multiversx/type/v1"
	"github.com/streamingfast/bstream"
	"github.com/streamingfast/dstore"
	pbbstream "github.com/streamingfast/pbgo/sf/bstream/v1"
	"github.com/urfave/cli"
	"google.golang.org/protobuf/proto"
)

var log = logger.GetOrCreate("checker")

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

	printOneBlockE(204)

	return nil
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

	fmt.Println(hash)
	_ = proxy
	return nil
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

func printOneBlockE(blockNum uint64) error {
	bstream.GetBlockReaderFactory = bstream.BlockReaderFactoryFunc(blockReaderFactory)
	bstream.GetBlockDecoder = bstream.BlockDecoderFunc(BlockDecoder)
	bstream.GetBlockWriterHeaderLen = 10
	bstream.GetBlockPayloadSetter = bstream.MemoryBlockPayloadSetter
	bstream.GetMemoizeMaxAge = 20 * time.Second
	str := "/home/elrond/go/src/github.com/ElrondNetwork/firehose-multiversx/devel/standard/firehose-data/storage/one-blocks" //"../devel/standard/firehose-data/storage/one-blocks" //viper.GetString("store")

	store, err := dstore.NewDBinStore(str)
	if err != nil {
		return fmt.Errorf("unable to create store at path %q: %w", store, err)
	}

	ctx := context.Background()
	var files []string
	filePrefix := fmt.Sprintf("%010d", blockNum)
	err = store.Walk(ctx, filePrefix, func(filename string) (err error) {
		files = append(files, filename)
		return nil
	})
	if err != nil {
		return fmt.Errorf("unable to find on block files: %w", err)
	}

	for _, filepath := range files {
		ppp := str + "/" + filepath + ".dbin.zst"

		_ = ppp
		reader, err := store.OpenObject(ctx, filepath)
		if err != nil {
			fmt.Printf("❌ Unable to read block filename %s: %s\n", ppp, err)
			return err
		}
		defer reader.Close()

		readerFactory, err := bstream.GetBlockReaderFactory.New(reader)
		if err != nil {
			fmt.Printf("❌ Unable to read blocks filename %s: %s\n", filepath, err)
			return err
		}

		//fmt.Printf("One Block File: %s\n", store.ObjectURL(filepath))

		block, err := readerFactory.Read()
		if err != nil {
			if err == io.EOF {
				break
			}
			return fmt.Errorf("reading block: %w", err)
		}

		if err = printBlock(block); err != nil {
			return err
		}

	}
	return nil
}

func printBlock(block *bstream.Block) error {
	nativeBlock := block.ToProtocol().(*pbmultiversx.Block)

	data, err := json.MarshalIndent(nativeBlock, "", "  ")
	if err != nil {
		return fmt.Errorf("json marshall: %w", err)
	}

	fmt.Println(string(data))

	return nil
}
