#!/bin/bash

# Compile `Setup` program
RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-unknown-linux-gnu

# Copy exectuable to `setup-deb/bin/` directory
cp target/x86_64-unknown-linux-gnu/release/setup setup-deb/bin/

# Build deb package
dpkg-deb --root-owner-group --build setup-deb

# Move deb package to `target/` directory
mv setup-deb.deb target/setup.deb