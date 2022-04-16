#!/bin/bash

set -e

if [[ "$EUID" = 0 ]]; then
    echo "Don't run this script as root"
    exit 1
fi

if ! command -v cargo; then
    echo "Installing rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    echo "Please start a new terminal session so that cargo is in your PATH"
    exit 1
fi

echo "Installing libsqlite3-dev"
sudo apt install libsqlite3-dev -y

cargo install diesel_cli --no-default-features --features sqlite

cargo install cargo-diff-tools

