#!/bin/sh

cargo clean
export OPENSSL_LIB_DIR=/usr/local/opt/openssl@1.1/lib/;
export OPENSSL_INCLUDE_DIR=/usr/local/opt/openssl@1.1/include;
export OPENSSL_STATIC=yes
export RUSTFLAGS="-Clink-arg=-Wl,--allow-multiple-definition"

cargo build --release
