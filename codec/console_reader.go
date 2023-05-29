package codec

import (
	"bufio"
	"encoding/hex"
	"fmt"
	"io"
	"strconv"
	"strings"
	"time"

	"github.com/gogo/protobuf/proto"
	"github.com/multiversx/firehose-multiversx/types"
	pbmultiversx "github.com/multiversx/firehose-multiversx/types/pb/sf/multiversx/type/v1"
	"github.com/multiversx/mx-chain-core-go/data/outport"
	"github.com/streamingfast/bstream"
	"go.uber.org/zap"
)

// ConsoleReader is what reads the `geth` output directly. It builds
// up some LogEntry objects. See `LogReader to read those entries .
type ConsoleReader struct {
	lines chan string
	close func()

	ctx  *parseCtx
	done chan interface{}

	logger *zap.Logger
}

func NewConsoleReader(logger *zap.Logger, lines chan string) (*ConsoleReader, error) {
	l := &ConsoleReader{
		lines:  lines,
		close:  func() {},
		done:   make(chan interface{}),
		logger: logger,
	}
	return l, nil
}

//todo: WTF?
func (r *ConsoleReader) Done() <-chan interface{} {
	return r.done
}

func (r *ConsoleReader) Close() {
	r.close()
}

type parsingStats struct {
	startAt  time.Time
	blockNum uint64
	data     map[string]int
	logger   *zap.Logger
}

func newParsingStats(logger *zap.Logger, block uint64) *parsingStats {
	return &parsingStats{
		startAt:  time.Now(),
		blockNum: block,
		data:     map[string]int{},
		logger:   logger,
	}
}

func (s *parsingStats) log() {
	s.logger.Info("reader block stats",
		zap.Uint64("block_num", s.blockNum),
		zap.Int64("duration", int64(time.Since(s.startAt))),
		zap.Reflect("stats", s.data),
	)
}

func (s *parsingStats) inc(key string) {
	if s == nil {
		return
	}
	k := strings.ToLower(key)
	value := s.data[k]
	value++
	s.data[k] = value
}

type parseCtx struct {
	currentBlock *pbmultiversx.Block
	stats        *parsingStats

	logger *zap.Logger
}

func newContext(logger *zap.Logger, height uint64) *parseCtx {
	return &parseCtx{
		currentBlock: &pbmultiversx.Block{
			Height: height,
		},
		stats: newParsingStats(logger, height),

		logger: logger,
	}
}

func (r *ConsoleReader) ReadBlock() (out *bstream.Block, err error) {
	block, err := r.next()
	if err != nil {
		return nil, err
	}

	return types.BlockFromProto(block)
}

const (
	LogPrefix     = "FIRE"
	LogBeginBlock = "BLOCK_BEGIN"
	LogEndBlock   = "BLOCK_END"
)

const (
	blockBeginChunks   = 1
	blockBeginNonceIdx = 0

	blockEndChunks       = 4
	blockEndNonceIdx     = 0
	blockEndPrevHashIdx  = 1
	blockEndTimestampIdx = 2
	blockEndProtoBytes   = 3
)

func (r *ConsoleReader) next() (out *pbmultiversx.Block, err error) {
	for line := range r.lines {
		if !strings.HasPrefix(line, LogPrefix) {
			continue
		}

		// This code assumes that distinct element do not contains space. This can happen
		// for example when exchanging JSON object (although we strongly discourage usage of
		// JSON, use serialized Protobuf object). If you happen to have spaces in the last element,
		// refactor the code here to avoid the split and perform the split in the line handler directly
		// instead.
		tokens := strings.Split(line[len(LogPrefix)+1:], " ")
		if len(tokens) < 2 {
			return nil, fmt.Errorf("invalid log line %q, expecting at least two tokens", line)
		}

		// Order the case from most occurring line prefix to least occurring
		switch tokens[0] {
		case LogBeginBlock:
			err = r.blockBegin(tokens[1:])
		case LogEndBlock:
			// This end the execution of the reading loop as we have a full block here
			return r.ctx.readBlockEnd(tokens[1:])
		default:
			if r.logger.Core().Enabled(zap.DebugLevel) {
				r.logger.Debug("skipping unknown deep mind log line", zap.String("line", line))
			}
			continue
		}

		if err != nil {
			chunks := strings.SplitN(line, " ", 2)
			return nil, fmt.Errorf("%s: %w (line %q)", chunks[0], err, line)
		}
	}

	r.logger.Info("lines channel has been closed")
	return nil, io.EOF
}

func (r *ConsoleReader) processData(reader io.Reader) error {
	scanner := r.buildScanner(reader)
	for scanner.Scan() {
		line := scanner.Text()
		r.lines <- line
	}

	if scanner.Err() == nil {
		close(r.lines)
		return io.EOF
	}

	return scanner.Err()
}

func (r *ConsoleReader) buildScanner(reader io.Reader) *bufio.Scanner {
	buf := make([]byte, 50*1024*1024)
	scanner := bufio.NewScanner(reader)
	scanner.Buffer(buf, 50*1024*1024)

	return scanner
}

// Format:
// FIRE BLOCK_BEGIN <NUM>
func (r *ConsoleReader) blockBegin(params []string) error {
	if err := validateChunk(params, blockBeginChunks); err != nil {
		return fmt.Errorf("invalid log line length: %w", err)
	}

	blockHeight, err := strconv.ParseUint(params[blockBeginNonceIdx], 10, 64)
	if err != nil {
		return fmt.Errorf("invalid block num: %w", err)
	}

	//Push new block
	r.ctx = newContext(r.logger, blockHeight)
	return nil
}

// Format:
// FIRE BLOCK_END <HEIGHT> <PREV_HASH> <TIMESTAMP> <PROTO-BYTES-FIREHOSE-BLOCK>
func (ctx *parseCtx) readBlockEnd(params []string) (*pbmultiversx.Block, error) {
	if err := validateChunk(params, blockEndChunks); err != nil {
		return nil, fmt.Errorf("invalid log line length: %w", err)
	}

	if ctx.currentBlock == nil {
		return nil, fmt.Errorf("current block not set")
	}

	blockHeight, err := strconv.ParseUint(params[blockEndNonceIdx], 10, 64)
	if err != nil {
		return nil, fmt.Errorf("failed to parse blockNum: %w", err)
	}
	if blockHeight != ctx.currentBlock.Height {
		return nil, fmt.Errorf("end block height does not match active block height, got block height %d but current is block height %d", blockHeight, ctx.currentBlock.Height)
	}

	timestamp, err := strconv.ParseUint(params[blockEndTimestampIdx], 10, 64)
	if err != nil {
		return nil, fmt.Errorf("failed to parse block timestamp: %w", err)
	}

	multiversxBlockBytes, err := hex.DecodeString(params[blockEndProtoBytes])
	if err != nil {
		return nil, err
	}

	multiversxBlock := &outport.OutportBlock{}
	err = proto.Unmarshal(multiversxBlockBytes, multiversxBlock)
	if err != nil {
		return nil, err
	}

	ctx.currentBlock.PrevHash = params[blockEndPrevHashIdx]
	ctx.currentBlock.Timestamp = timestamp
	ctx.currentBlock.MultiversxBlock = multiversxBlock

	ctx.logger.Debug("console reader read block",
		zap.Uint64("height", ctx.currentBlock.Height),
		zap.String("hash", hex.EncodeToString(ctx.currentBlock.MultiversxBlock.BlockData.HeaderHash)),
		zap.String("prev_hash", ctx.currentBlock.PrevHash),
	)

	return ctx.currentBlock, nil
}

func validateChunk(params []string, count int) error {
	if len(params) != count {
		return fmt.Errorf("%d fields required but found %d", count, len(params))
	}
	return nil
}
