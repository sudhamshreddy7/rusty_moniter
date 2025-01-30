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
echo "Enter log file path:"
read LOG_FILE_PATH
export LOG_FILE_PATH
nohup cargo run > /dev/null 2>&1 &