#!/bin/bash
clear

cat circuit/Dockerfile

cat circuit/foundry.toml

docker compose up circuit --build
if [ $? -ne 0 ]; then
    echo "❌ Failed to build the circuit"
    exit 1
fi

echo "✅ Successfully built the circuit"