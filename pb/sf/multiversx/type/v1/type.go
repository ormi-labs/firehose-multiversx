package pbmultiversx

import (
	"encoding/hex"
	"time"

	firecore "github.com/streamingfast/firehose-core"
)

var _ firecore.Block = (*HyperOutportBlock)(nil)

func (b *HyperOutportBlock) GetFirehoseBlockID() string {
	return hex.EncodeToString(b.MetaOutportBlock.BlockData.HeaderHash)
}

func (b *HyperOutportBlock) GetFirehoseBlockNumber() uint64 {
	return b.MetaOutportBlock.BlockData.Header.GetNonce()
}

func (b *HyperOutportBlock) GetFirehoseBlockParentID() string {
	if hex.EncodeToString(b.MetaOutportBlock.BlockData.Header.PrevHash) == "" {
		return ""
	}

	return hex.EncodeToString(b.MetaOutportBlock.BlockData.Header.PrevHash)
}

func (b *HyperOutportBlock) GetFirehoseBlockParentNumber() uint64 {
	var previousNum uint64
	if b.MetaOutportBlock.BlockData.Header.GetNonce() == 0 {
		previousNum = 0
	}

	previousNum = b.MetaOutportBlock.BlockData.Header.GetNonce() - 1

	return previousNum
}

func (b *HyperOutportBlock) GetFirehoseBlockTime() time.Time {
	return time.Unix(0, int64(b.MetaOutportBlock.BlockData.Header.GetTimeStamp())).UTC()
}

func (b *HyperOutportBlock) GetFirehoseBlockLIBNum() uint64 {
	return b.MetaOutportBlock.HighestFinalBlockNonce
}
