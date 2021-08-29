#!/bin/sh
MODULES="$1"
cargo test ${MODULES} -- --nocapture
