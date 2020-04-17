#!/bin/bash
current=$(pwd)

# Build Client
cd frontend 
echo $(pwd)
cargo web deploy --target=wasm32-unknown-unknown --release

# Build Server
cd $current/server
cp $current/frontend/target/deploy/app.* ./

cargo build

