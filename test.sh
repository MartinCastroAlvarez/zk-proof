#!/bin/bash

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

# Set the base URL
BASE_URL="http://localhost:3030"

# Test 1: Health check
echo "Running Test 1: Health check..."
response=$(curl -s -X GET "$BASE_URL/health")
if [ "$response" != "\"Ok\"" ]; then
    echo "❌ Test 1 failed: expected 'Ok', got '$response'"
    exit 1
fi
echo "✅ Test 1 passed"
echo

# Test 2: Generate a proof
echo "Running Test 2: Generate a proof..."
response=$(curl -s -X POST "$BASE_URL/generate" -H 'content-type: application/json' -d '{"a": "10", "b": "20"}')
if [ $? -ne 0 ]; then
    echo "❌ Test 2 failed"
    exit 1
fi
echo "$response"
echo

# Extract the proof from Test 2's response for use in Test 3
proof=$(echo $response | jq -r '.proof')
echo "Proof: $proof"
if [ -z "$proof" ]; then
    echo "❌ Test 2 failed"
    exit 1
fi
echo "✅ Test 2 passed"
echo

# Test 3: Verify a proof
echo "Running Test 3: Verify the proof..."
response=$(curl -s -X POST "$BASE_URL/verify" -H 'content-type: application/json' -d '{"proof": "'"$proof"'", "public_input": "30"}')
if [ $? -ne 0 ]; then
    echo "❌ Test 3 failed"
    exit 1
fi
echo "$response"
echo
if ! $(echo $response | jq -e '.is_valid == true' >/dev/null); then
    echo "❌ Test 3 failed"
    exit 1
fi
echo "✅ Test 3 passed"
echo

# Test 4: Verify an invalid proof
echo "Running Test 4: Verify an invalid proof..."
response=$(curl -s -X POST "$BASE_URL/verify" -H 'content-type: application/json' -d "{\"proof\": \"$proof\", \"public_input\": \"31\"}")
if [ $? -ne 0 ]; then
    echo "❌ Test 4 failed"
    exit 1
fi
echo "Response: $response"
echo
if ! $(echo $response | jq -e '.is_valid == false' >/dev/null); then
    echo "❌ Test 4 failed (Invalid proof should not be valid)"
    exit 1
fi
echo "✅ Test 4 passed"
echo

# Test 5: Export the verifying key
echo "Running Test 5: Export the verifying key..."
response=$(curl -s -X GET "$BASE_URL/vk")
if [ $? -ne 0 ]; then
    echo "❌ Test 5 failed"
    exit 1
fi
echo "$response"
echo
if ! $(echo $response | jq -e '.vk != null' >/dev/null); then
    echo "❌ Test 5 failed"
    exit 1
fi
echo "✅ Test 5 passed"
echo

# Extract the VK for Test 6
vk=$(echo $response | jq -r '.vk')
echo "VK: $vk"

# Test 6: Set credentials
echo "Running Test 6: Set credentials..."
response=$(curl -s -X POST "$BASE_URL/auth" -H 'content-type: application/json' -d '{"address": "'"$ADDRESS"'", "secret_key": "'"$SECRET_KEY"'"}')
if [ $? -ne 0 ]; then
    echo "❌ Test 6 failed"
    exit 1
fi
if ! $(echo $response | jq -e '.address == "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"' >/dev/null); then
    echo "❌ Test 6 failed: expected 'Credentials set', got '$response'"
    exit 1
fi
echo "✅ Test 6 passed"
echo

# Test 7: Get public address
echo "Running Test 7: Get public address..."
response=$(curl -s -X GET "$BASE_URL/auth")
if [ $? -ne 0 ]; then
    echo "❌ Test 7 failed"
    exit 1
fi
if ! $(echo $response | jq -e '.address == "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"' >/dev/null); then
    echo "❌ Test 7 failed: expected '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266', got '$response'"
    exit 1
fi
echo "✅ Test 7 passed"
echo

# Get the public address
address=$(echo $response | jq -r '.address')
echo $response
echo "Public address: $address"
echo ""

# Test 8: Deploy the contracts
echo "Running Test 8: Deploy the contracts..."
response=$(curl -s -X POST "$BASE_URL/deploy")
if [ $? -ne 0 ]; then
    echo "❌ Test 8 failed"
    exit 1
fi
echo "Response: $response"
if ! $(echo $response | jq -e '.verifier_address != null' >/dev/null); then
    echo "❌ Test 8 failed"
    exit 1
fi
if ! $(echo $response | jq -e '.manager_address != null' >/dev/null); then
    echo "❌ Test 8 failed"
    exit 1
fi
echo "✅ Test 8 passed"
echo

# All tests passed successfully!
echo "✅ All tests passed successfully!"
