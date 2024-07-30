#!/bin/bash

# Build the release version of the project
cargo build --release

# Create a symbolic link to the binary
ln -s $(pwd)/target/release/todo-cli /usr/local/bin/tc
