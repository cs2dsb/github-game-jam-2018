#!/usr/bin/env bash

#Using nightly so shred can tell us what resource was missing
RUST_BACKTRACE=1 cargo +nightly run --release --bin main --features nightly