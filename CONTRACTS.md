# Smart Contract Files

This folder contains the smart contracts that verify zero-knowledge proofs and manage access control.

| File | Description |
|------|-------------|
| [`src/ZkManager.sol`](./src/ZkManager.sol) | The main contract that checks proofs and manages who can do what. |
| [`test/ZkManager.t.sol`](./test/ZkManager.t.sol) | Tests to make sure the contract works correctly. |
| [`scripts/test.sh`](./scripts/test.sh) | Script to run the tests. |
| [`scripts/deploy.sh`](./scripts/deploy.sh) | Script to deploy the contract. |
| [`scripts/entrypoint.sh`](./scripts/entrypoint.sh) | Script to run the tests and deploy the contract. |
| [`Dockerfile`](./Dockerfile) | Instructions to build and test the contract in a container. |

## Access Control

The contract uses three levels of access: admins (who can do everything), authorities (who can change which contract checks proofs), and maintainers (who can upgrade the contract). This system makes sure that different people have different permissions, making the system more secure.

## Upgradability

When we need to upgrade the contract, the new contract must first show it can handle the current state (how many proofs we've checked). It does this in two steps: first trying to confirm it has the right state, and if not, importing the state. After upgrade, the old contract stops working and all calls to it will fail.

To do that, call:

```solidity
zkManager.upgradeContract(address(newContract));
```

## RISC0-Verification

RISC0 helps us check complex math without doing it on the blockchain (which would be very expensive). Instead, someone does the math elsewhere and gives us a proof that they did it correctly. Our contract then checks this proof, which is much cheaper than doing the original math. This is like having someone solve a hard puzzle and just checking their answer instead of solving it yourself.

To verify a proof, call:

```solidity
zkManager.batchValidateProofs(proofs);
```

## Gas Cost Optimization

The contract uses batch operations to save gas. Instead of checking proofs one at a time, we can check many proofs in a single transaction with `batchValidateProofs`. This means users pay the transaction fee only once for many proofs, making it much cheaper than sending each proof separately.

For example, instead of:

```solidity
zkManager.validateProof(proof1);
zkManager.validateProof(proof2);
```

We can do:

```solidity
zkManager.batchValidateProofs([proof1, proof2]);
```

This saves gas because it only costs once to send the transaction.
