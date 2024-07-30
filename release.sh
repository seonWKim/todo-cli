#!/bin/bash

# Remove build target directory if exists
rm -rf target

# Build the release version of the project
cargo build --release

# Ensure the local bin directory exists
mkdir -p $HOME/.local/bin

# Remove the existing symbolic link if exists
rm -f $HOME/.local/bin/tc

# Create a symbolic link to the binary
ln -s $(pwd)/target/release/todo-cli $HOME/.local/bin/tc

# Ensure the local bin directory is in the PATH
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    if [[ -f $HOME/.zshrc ]]; then
        if ! grep -q 'export PATH=$HOME/.local/bin:$PATH' $HOME/.zshrc; then
            echo 'export PATH=$HOME/.local/bin:$PATH' >> $HOME/.zshrc
            echo "Please run: source $HOME/.zshrc"
        fi
    elif [[ -f $HOME/.bashrc ]]; then
        if ! grep -q 'export PATH=$HOME/.local/bin:$PATH' $HOME/.bashrc; then
            echo 'export PATH=$HOME/.local/bin:$PATH' >> $HOME/.bashrc
            echo "Please run: source $HOME/.bashrc"
        fi
    else
        echo "No .bashrc or .zshrc file found. Please create one and add the following line:"
        echo 'export PATH=$HOME/.local/bin:$PATH'
    fi
fi
