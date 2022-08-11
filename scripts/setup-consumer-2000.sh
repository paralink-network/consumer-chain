#!/bin/bash

# Location independent running
scriptDir=$(dirname -- "$(readlink -f -- "${BASH_SOURCE[0]}")")
RELAY_CHAIN_SPEC="$scriptDir"/../assets/polkadot-relay-chain-spec.json
PLAIN_SPEC="$scriptDir"/../assets/consumer-parachain-plain.json
RAW_SPEC="$scriptDir"/../assets/consumer-parachain-2000-raw.json
WASM_VALIDATION="$scriptDir"/../assets/consumer-2000-wasm
GENESIS="$scriptDir"/../assets/consumer-2000-genesis

# Build chain
(cd "$scriptDir"; cargo build --release)

# Generate plain spec
"$scriptDir"/../target/release/consumer-parachain build-spec --disable-default-bootnode > "$PLAIN_SPEC"

# Change 1000 -> 2000 two times in the resulting json
sed -i 's@para_id": 1000@para_id": 2000@g' "$PLAIN_SPEC"
sed -i 's@parachainId": 1000@parachainId": 2000@g' "$PLAIN_SPEC"

# Reserve the ParaId through https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/parachains/parathreads

# Generate raw spec from updated plain file
"$scriptDir"/../target/release/consumer-parachain build-spec --chain "$PLAIN_SPEC" --raw --disable-default-bootnode > "$RAW_SPEC"

# Generate validation function and genesis state
"$scriptDir"/../target/release/consumer-parachain export-genesis-wasm --chain "$RAW_SPEC" > "$WASM_VALIDATION"
"$scriptDir"/../target/release/consumer-parachain export-genesis-state --chain "$RAW_SPEC"> "$GENESIS"

# Start the chain
"$scriptDir"/../target/release/consumer-parachain --collator --alice --force-authoring --tmp --port 40335 --ws-port 9946 --rpc-external --ws-external --rpc-cors all --rpc-methods=Unsafe --chain "$RAW_SPEC" -- --execution wasm --chain "$RELAY_CHAIN_SPEC" --port 30335

