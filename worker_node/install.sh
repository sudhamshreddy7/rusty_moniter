#!/bin/bash
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
clear
export API_KEY=$(openssl rand -hex 16)
echo "Your API key:"
printenv API_KEY
cargo build
cargo run &