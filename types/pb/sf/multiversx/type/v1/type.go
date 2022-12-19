package pbmultiversx

import (
	"time"
)

func (b *WrappedMultiversxBlock) ID() string {
	return b.Hash
}

func (b *WrappedMultiversxBlock) Number() uint64 {
	return b.Height
}

func (b *WrappedMultiversxBlock) PreviousID() string {
	return b.PrevHash
}

func (b *WrappedMultiversxBlock) Time() time.Time {
	return time.Unix(0, int64(b.Timestamp)).UTC()
}
