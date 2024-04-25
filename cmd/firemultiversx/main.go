package main

import (
	firecore "github.com/streamingfast/firehose-core"
	fhCmd "github.com/streamingfast/firehose-core/cmd"

	pbmultiversx "github.com/multiversx/firehose-multiversx/pb/sf/multiversx/type/v1"
)

func main() {
	fhCmd.Main(&firecore.Chain[*pbmultiversx.HyperOutportBlock]{
		ShortName:            "multiversx",
		LongName:             "Multiversx",
		ExecutableName:       "multiversx",
		FullyQualifiedModule: "github.com/multiversx/firehose-multiversx",
		Version:              version,

		FirstStreamableBlock: 0,

		BlockFactory:         func() firecore.Block { return new(pbmultiversx.HyperOutportBlock) },
		ConsoleReaderFactory: firecore.NewConsoleReader,

		Tools: &firecore.ToolsConfig[*pbmultiversx.HyperOutportBlock]{},
	})
}

// Version value, injected via go build `ldflags` at build time, **must** not be removed or inlined
var version = "dev"
