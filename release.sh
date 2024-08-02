#!/bin/bash

rm todo-cli
rm -rf target

# Build the release version of the project
cargo build --release
mv target/release/todo-cli .

# Remove the existing symbolic link if exists
rm -f $HOME/.tc/bin/tc
