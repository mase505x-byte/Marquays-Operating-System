#!/bin/bash
cargo +nightly build \
	--target targets/x86_64-marq-none.json \
	-Z build-std=core,alloc \
	-Z build-std-features=compiler-builtins-mem
# If you are building for x86_64, you can use the built-in bare-metal target
rustup component add rust-src
# build the kernel
cargo build --target x86_64-unkown-none --release
