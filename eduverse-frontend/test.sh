#!/bin/sh

wasm-pack test --headless --firefox

# filters
# wasm-pack test --headless --firefox . -- get_config