#!/bin/bash

# Test if the number of arguments is correct
if test "$#" -ne 1; then
    echo "Version number is missing."
    echo "Use the version number as an argument."
    exit 1
fi

PACKAGENAME=setup_$1_amd64.deb

# Compile `Setup` program
RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-unknown-linux-gnu

# Copy exectuable to `deb-packager/bin/` directory
mkdir deb-packager/bin
cp target/x86_64-unknown-linux-gnu/release/setup deb-packager/bin/

# Copy Jsonschema file to `deb-packager/usr/share/setup/` directory
mkdir -p deb-packager/usr/share/setup/
cp jsonschema.json deb-packager/usr/share/setup/projectfileschema.json

# Build deb package
dpkg-deb --root-owner-group --build deb-packager $PACKAGENAME

# Move deb package to `target/` directory
mv $PACKAGENAME target/

# Remove temp directories
rm -rf deb-packager/bin
rm -rf deb-packager/usr