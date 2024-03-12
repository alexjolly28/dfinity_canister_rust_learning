#!/usr/bin/env zsh

function generate_did() {
  local canister=$1
  canister_root="."

  cargo build --manifest-path="$canister_root/Cargo.toml" \
      --target wasm32-unknown-unknown \
      --release --package "$canister"

  candid-extractor "target/wasm32-unknown-unknown/release/$canister.wasm" > "src/$canister/$canister.did"
}

CANISTERS=queue_backend

for canister in $(echo $CANISTERS | sed "s/,/ /g")
do
    generate_did "$canister"
done