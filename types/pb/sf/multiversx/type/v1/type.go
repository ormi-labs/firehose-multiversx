package pbmultiversx

import (
	"encoding/hex"
	"time"
)

func (b *Block) ID() string {
	return hex.EncodeToString(b.MultiversxBlock.BlockData.HeaderHash)
}

func (b *Block) Number() uint64 {
	return b.Height
}

func (b *Block) PreviousID() string {
	return b.PrevHash
}

func (b *Block) Time() time.Time {
	return time.Unix(0, int64(b.Timestamp)).UTC()
}
