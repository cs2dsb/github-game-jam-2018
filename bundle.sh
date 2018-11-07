#!/usr/bin/env bash

cargo +nightly build --release --bin main --features nightly
rm -rf bundle
rm -f bundle.zip
mkdir bundle
cp -r assets bundle/
cp -r resources bundle/
cp target/release/main bundle/
zip -r bundle.zip bundle/*