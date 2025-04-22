#!/bin/bash

set -e

# echo "dist dir: $TRUNK_DIST_DIR"

cd modules/rain-game
if [[ "$TRUNK_PROFILE" == "release" ]]; then
  BUILD_MODE="--release"
fi
cargo b $BUILD_MODE --target=wasm32-unknown-unknown
cd ../..

mkdir -p $TRUNK_STAGING_DIR/games
cp modules/rain-game/target/wasm32-unknown-unknown/$TRUNK_PROFILE/rain-game.wasm $TRUNK_STAGING_DIR/games/rain-game.wasm
