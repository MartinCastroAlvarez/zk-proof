# ZK Tests

This document describes how to test the ZK API, assuming that the API is running on `http://localhost:3030`.

#### Test 1: To generate a proof, run the following command:

```bash
curl -iX POST 'http://localhost:3030/generate' -H 'content-type: application/json' -d '{"a": "10", "b": "20"}'
```

Response:

```bash
HTTP/1.1 200 OK
content-type: application/json
content-length: 278
date: Sun, 09 Feb 2025 19:46:48 GMT

{"proof":"awqqekC1YE7VPFGEaZfg1yuM1dhcMwQ9fzGjjtOc7KxG1jtshViS2Gi6L2GHDE1A6iWsdYSCvVEraTXNXkj0LphX0FKhxJ0W76795IwbISd1LDw94a53wFOzqOncIS6ahYhWYeos/suYARTB1DI0Kt5DjjFPCmnaL8F4gfT95hc=","public_input":"Fp256 \"(000000000000000000000000000000000000000000000000000000000000001E)\""}
```

#### Test 2: To verify a proof, run the following command:

```bash
PROOF=$(curl -sX POST 'http://localhost:3030/generate' -H 'content-type: application/json' -d '{"a": "10", "b": "20"}' | jq -r '.proof');
curl -iX POST 'http://localhost:3030/verify' -H 'content-type: application/json' -d '{"proof": "'"${PROOF}"'", "public_input": "30"}'
```

Response:

```bash
HTTP/1.1 200 OK
content-type: application/json
content-length: 18

{"is_valid":true}
```

#### Test 3: To verify an invalid proof, run the following command:

```bash
PROOF=$(curl -sX POST 'http://localhost:3030/generate' -H 'content-type: application/json' -d '{"a": "10", "b": "20"}' | jq -r '.proof');
curl -iX POST 'http://localhost:3030/verify' -H 'content-type: application/json' -d '{"proof": "'"${PROOF}"'", "public_input": "31"}'
```

#### Test 4: To export the verifying key to a base64 string, run the following command:

```bash
curl -iX GET 'http://localhost:3030/vk'
```

Response:

```bash
HTTP/1.1 200 OK
content-type: application/json
content-length: 398
date: Sun, 09 Feb 2025 20:10:29 GMT

{"vk": "uBeicrclTefXi6NSkEg3pN3s2zFUmmsjLs6qeNlAPZZ6JwLcJRMbz6ta6tx7iwLK5QgGhSxTRVGiuAWaJpJaKEVcUdoISbJJYYK64wrgkRVVxAdLYJoH89Z2NBo2H/SVHRdh6kedGaAl8n/QO0/B1QQN7IN3AgimvKa90gvbQiv3rEJcHDHBxCd6Qr1sXuAOHSP6BPtnmVptQA20Ii7OLqgUIRSIxBFSGQRu5xkRlQV/qSu+1KLiynhxCBeFLj0C6nlS1YxGbZIT5GITW/9m0a2AYrNY05Ax6MMRwWZmkqkCAAAAAAAAAF0uo9rOvrhzQ4wwoYxO0d8uSgBki4wZs5alfA3ooqIJYiKDJK4cwlXGNjOLD6Jb19zorYWC4JEmiiayNRT68AY="}
```

#### Test 5: To set credentials, run the following command:

```bash
curl -iX POST 'http://localhost:3030/auth' -H 'content-type: application/json' -d '{"address": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266", "secret_key": "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"}'
```

Response:

```bash
HTTP/1.1 200 OK
content-type: application/json
content-length: 14

{"address":"0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"}
```

#### Test 6: To get the public address, run the following command:

```bash
curl -iX GET 'http://localhost:3030/auth'
```

Response:

```bash
HTTP/1.1 200 OK
content-type: application/json
content-length: 40

{"address":"0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"}
```

Alternativelly, you can run the [test.sh](test.sh) script to test the API.
