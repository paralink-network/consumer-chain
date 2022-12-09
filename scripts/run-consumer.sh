#!/bin/bash

# Runs the parachain, assumes relay chain is already running, see paralink-xcm repo `make run_relay` command

scriptDir=$(dirname -- "$(readlink -f -- "${BASH_SOURCE[0]}")")
RELAY_CHAIN_SPEC="$scriptDir"/../assets/polkadot-relay-chain-spec.json
RAW_SPEC="$scriptDir"/../assets/consumer-parachain-2000-raw.json

# Build chain
(cd "$scriptDir"; cargo build --release) || exit $?

echo "$scriptDir"

# Start the chain
 "$scriptDir"/../target/release/consumer-parachain --collator --alice --force-authoring --tmp --port 40335 --ws-port 9946 --rpc-external --ws-external --rpc-cors all --rpc-methods=Unsafe --chain "$RAW_SPEC" -- --execution native --chain "$RELAY_CHAIN_SPEC" --port 30335
