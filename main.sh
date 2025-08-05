#!/bin/sh
cargo build --release
./target/release/rust_static-site
cd docs && python3 -m http.server 8888
