#!/bin/bash
if ! command -v rustc &>/dev/null; then
    echo "Rust is not installed. Installing..."

    # Install Rust using rustup
    curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh

    echo "Rust has been installed."
else
    echo "Rust is already installed."
fi

clear
export API_KEY=$(openssl rand -hex 16)
echo "Your API key:"
printenv API_KEY
cargo run &