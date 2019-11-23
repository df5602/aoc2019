#!/bin/bash

cp -Rv template $1

# Use GNU sed on Mac OS X because I haven't got enough hair to pull out to make standard sed work..
# Install with '$ brew install gnu-sed'

# Replace name in crate Cargo.toml
gsed -i "s/^name = .*/name = \"$1\"/" $1/Cargo.toml

# Add crate to workspace
gsed -i "s/^]$/    \"$1\",\n]/" Cargo.toml