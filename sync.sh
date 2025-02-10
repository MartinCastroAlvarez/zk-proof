#!/bin/bash

clear

docker ps --format '{{.Names}}'

CONTAINER_NAME=$(docker ps --format '{{.Names}}' | grep circuit | head -n 1)
OUTPUT_DIR="/tmp"

echo "CONTAINER_NAME: $CONTAINER_NAME"
if [ -z "$CONTAINER_NAME" ]; then
    echo "❌ No container found"
    exit 1
fi

docker exec -it $CONTAINER_NAME ls -la /tmp

# Extract files from the running container
docker cp "${CONTAINER_NAME}:/tmp/ZkManager.abi" "$OUTPUT_DIR/ZkManager.abi"
if [ $? -ne 0 ]; then
    echo "❌ Failed to extract ZkManager.abi"
    exit 1
fi
echo "✅ ZkManager.abi extracted"

docker cp "${CONTAINER_NAME}:/tmp/ZkManager.bin" "$OUTPUT_DIR/ZkManager.bin"
if [ $? -ne 0 ]; then
    echo "❌ Failed to extract ZkManager.bin"
    exit 1
fi
echo "✅ ZkManager.bin extracted"

docker cp "${CONTAINER_NAME}:/tmp/ZkVerifier.abi" "$OUTPUT_DIR/ZkVerifier.abi"
if [ $? -ne 0 ]; then
    echo "❌ Failed to extract ZkVerifier.abi"
    exit 1
fi
echo "✅ ZkManager.abi extracted"

docker cp "${CONTAINER_NAME}:/tmp/ZkVerifier.bin" "$OUTPUT_DIR/ZkVerifier.bin"
if [ $? -ne 0 ]; then
    echo "❌ Failed to extract ZkVerifier.bin"
    exit 1
fi

echo "✅ ZkVerifier.bin extracted"

wc -l "$OUTPUT_DIR/ZkManager.abi"
wc -l "$OUTPUT_DIR/ZkManager.bin"
wc -l "$OUTPUT_DIR/ZkVerifier.abi"
wc -l "$OUTPUT_DIR/ZkVerifier.bin"

echo "✅ Successfully extracted ZkManager.abi, ZkManager.bin, ZkVerifier.abi, and ZkVerifier.bin"
