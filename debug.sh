#!/usr/bin/env bash

#Using nightly so shred can tell us what resource was missing
cargo +nightly build --bin main --features nightly
CARGO_MANIFEST_DIR=. lldb -s lldb_set_breakpoint_run target/debug/main