#!/bin/bash

set -e

# echo "dist dir: $TRUNK_DIST_DIR"

cd modules/rain-game
trunk build --release
cd ../..

mkdir -p $TRUNK_STAGING_DIR/games/rain-game
cp -r modules/rain-game/dist/* $TRUNK_STAGING_DIR/games/rain-game/
