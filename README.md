# Risc Zero

Risc Zero Rust App

![wallpaper.jpg](./wallpaper.jpg)

## Overview

This project demonstrates a zero-knowledge proof system using *RiscZero*'s *zkVM*. It compiles a guest program located in the `./methods/guest` directory into an *ELF* binary, which is then executed by a host Rust application that also runs a Warp server. The server exposes endpoints to remotely trigger `zkVM` executions, allowing users to submit inputs, obtain the computed result along with a cryptographic proof of execution, and ultimately validate that proof on-chain. The entire process—from building the guest ELF to running the host server—is containerized using Docker, ensuring a reproducible and isolated environment for development and deployment.

## References

- [zkVM Installation](https://dev.risczero.com/api/zkvm/install)
- [zkVM Hello World](https://dev.risczero.com/api/zkvm/tutorials/hello-world)
- [Verifier Contracts](https://dev.risczero.com/api/blockchain-integration/contracts/verifier)
- [cargo-risczero](https://github.com/risc0/risc0/blob/main/risc0/cargo-risczero/README.md)
- [Risc Zero validation with Bonsai on Ethereum](https://dev.risczero.com/api/blockchain-integration/bonsai-on-eth)
- [Risc Zero verification contracts](https://dev.risczero.com/api/blockchain-integration/contracts/verifier)

## Setup

Simply run the following command to build and run the project:

```bash
docker-compose up
```

## Build

Build the guest ELF file in the Docker container running:

```bash
docker-compose exec zk cargo risczero build --manifest-path methods/guest/Cargo.toml 
docker-compose exec zk cp target/riscv-guest/riscv32im-risc0-zkvm-elf/docker/zk_guest/zk-guest ./src/GUEST.elf
```

The output will be something like this:

```bash
Image ID: 72628b77e7251ca8e4b4e8342ac213a90fe5be3a3a6ad76bebf3887478b7a914
```

Make sure you update the `IMAGE_ID` in the `docker-compose.yml` file with the output from the above command.

Next, to build the Rust host program, run the following command:

```bash
docker-compose build
```

## Deployment

To deploy the project, deploy the Docker image to a cloud provider of your choice.

## Testing

Check the health of the server:

```bash
curl -iX GET "http://0.0.0.0:3030/health"
```

You should see something like this:

```bash
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 2
date: Mon, 17 Feb 2025 16:08:57 GMT

OK
```

Call the Fibonacci function:

```bash
curl -X POST "http://0.0.0.0:3030/fib/4" > output.json
```

Now, inspect the output:

```bash
cat output.json
```

You should see something like this:

```json
{
    "result": 3,
    "proof": "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef..."
}
```
