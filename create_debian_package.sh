#!/bin/bash

# Compile `Setup` program
cargo build --release

# Copy exectuable to `setup-deb/bin/` directory
cp target/release/setup setup-deb/bin/

# Build deb package
dpkg-deb --root-owner-group --build setup-deb

# Move deb package to `target/` directory
mv setup-deb.deb target/setup.deb