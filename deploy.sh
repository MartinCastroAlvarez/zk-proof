#!/bin/bash

clear

# Get the name of the running Docker container
docker ps --format '{{.Names}}'
CONTAINER_NAME=$(docker ps --format '{{.Names}}' | grep devnet | head -n 1)

echo "Using Container: $CONTAINER_NAME"
if [ -z "$CONTAINER_NAME" ]; then
    echo "❌ No suitable container found"
    exit 1
fi

# Get the first account's address from inside the container
ADDRESS=$(docker logs $CONTAINER_NAME | grep "(0)" | tail -2 | awk '{print $2}' | head -n 1)

# Get the first account's private key from inside the container
SECRET_KEY=$(docker logs $CONTAINER_NAME | grep "(0)" | tail -2 | awk '{print $2}' | tail -n 1)

echo "Using Anvil Account:"
echo "Address: $ADDRESS"
echo "Private Key: $SECRET_KEY"

if [ -z "$ADDRESS" ] || [ -z "$SECRET_KEY" ]; then
    echo "❌ Failed to get Anvil account"
    exit 1
fi
if [ "$ADDRESS" == "null" ] || [ "$SECRET_KEY" == "null" ]; then
    echo "❌ Failed to get Anvil account"
    exit 1
fi

# Set directory variables
OUTPUT_DIR="/tmp"
HOST_DIR="/tmp"

# Copy .sol files to the container
docker cp "circuit/ZkManager.sol" "${CONTAINER_NAME}:/tmp/ZkManager.sol"
if [ $? -ne 0 ]; then
    echo "❌ Failed to copy ZkManager.sol to container"
    exit 1
fi
docker cp "circuit/ZkVerifier.sol" "${CONTAINER_NAME}:/tmp/ZkVerifier.sol"
if [ $? -ne 0 ]; then
    echo "❌ Failed to copy ZkVerifier.sol to container"
    exit 1
fi

echo "✅ Files copied to container"

# Deploying OpenZeppelin on the container
# docker exec $CONTAINER_NAME sh -c "npm cache clean --force && cd /tmp && npm init -y && npm install @openzeppelin/contracts"
# if [ $? -ne 0 ]; then
#     echo "❌ Failed to install openzeppelin-contracts"
#     exit 1
# fi
# docker exec $CONTAINER_NAME sh -c "ls /tmp/node_modules/@openzeppelin/contracts/access"
# if [ $? -ne 0 ]; then
#     echo "❌ Failed to install openzeppelin-contracts"
#     exit 1
# fi
# echo "✅ OpenZeppelin installed successfully"
# echo 

# Generating the verification key
echo '[
  "123",
  "456",
  "[789, 1011]",
  "[1213, 1415]",
  "[1617, 1819]",
  "[2021, 2223]",
  "[2425, 2627]",
  "[2829, 3031]",
  "3233",
  "3435",
  "3637",
  "3839"
]' > /tmp/verification_key.json

# Copying the verification key to the container
docker cp /tmp/verification_key.json "${CONTAINER_NAME}:/tmp/verification_key.json"
if [ $? -ne 0 ]; then
    echo "❌ Failed to copy verification_key.json to container"
    exit 1
fi

# Deploy the contract using Forge
DEPLOY_COMMAND="forge create \
    --rpc-url http://localhost:8545 \
    --private-key $SECRET_KEY \
    --constructor-args-path /tmp/verification_key.json \
    /tmp/ZkVerifier.sol:ZkVerifier \
    --broadcast"
echo "Deploying the contract using Forge with $DEPLOY_COMMAND"

# Execute the command in Docker and capture the output
DEPLOY_OUTPUT=$(docker exec $CONTAINER_NAME sh -c "$DEPLOY_COMMAND")
if [ $? -ne 0 ]; then
    echo "❌ Failed to deploy ZkVerifier"
    echo "$DEPLOY_OUTPUT"
    exit 1
fi
echo "$DEPLOY_OUTPUT"
echo "✅ ZkVerifier deployed successfully"

# Extract and print the contract address
VERIFIER_ADDRESS=$(echo "$DEPLOY_OUTPUT" | grep "Deployed to:" | awk '{print $3}')
if [ $? -ne 0 ]; then
    echo "❌ Deployment failed"
    exit 1
fi
if [ -z "$VERIFIER_ADDRESS" ]; then
    echo "❌ Failed to extract verifier contract address"
    exit 1
fi
echo "✅ Verifier Contract deployed successfully"
echo "Contract Address: $VERIFIER_ADDRESS"

# Generating the arguments of the second constructor
echo '[
    "'"$VERIFIER_ADDRESS"'"
]' > /tmp/manager_args.json

# Copying the manager arguments to the container
docker cp /tmp/manager_args.json "${CONTAINER_NAME}:/tmp/manager_args.json"
if [ $? -ne 0 ]; then
    echo "❌ Failed to copy manager_args.json to container"
    exit 1
fi

# Deploy the ZkManager contract using Forge
DEPLOY_MANAGER_COMMAND="forge create \
    --rpc-url http://localhost:8545 \
    --private-key $SECRET_KEY \
    --constructor-args-path /tmp/manager_args.json \
    /tmp/ZkManager.sol:ZkManager \
    --remappings '@openzeppelin=/tmp/node_modules/@openzeppelin' \
    --broadcast"
echo "Deploying ZkManager contract using Forge..."
DEPLOY_MANAGER_OUTPUT=$(docker exec $CONTAINER_NAME sh -c "$DEPLOY_MANAGER_COMMAND")
echo "$DEPLOY_MANAGER_OUTPUT"

# Extract and print the ZkManager contract address
MANAGER_ADDRESS=$(echo "$DEPLOY_MANAGER_OUTPUT" | grep "Deployed to:" | awk '{print $3}')
if [ $? -ne 0 ]; then
    echo "❌ Failed to extract manager contract address"
    exit 1
fi
if [ -z "$MANAGER_ADDRESS" ]; then
    echo "❌ Failed to extract manager contract address"
    exit 1
fi
echo "✅ ZkManager deployed successfully"
echo "Manager Contract Address: $MANAGER_ADDRESS"
