#!/bin/bash

# Remove build target directory if exists
rm -rf target

# Build the release version of the project
cargo build --release

# Remove the existing symbolic link if exists
rm -f /usr/local/bin/tc

# Create a symbolic link to the binary
ln -s $(pwd)/target/release/todo-cli /usr/local/bin/tc
