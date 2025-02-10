// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/access/AccessControl.sol";

interface IRiscZeroVerifier {
    function verifyProofs(bytes[] calldata proofs) external returns (bool[] memory);
}

interface IUpgradeable {
    function importState(uint256 state) external;
    function confirmState(uint256 state) external returns (bool);
}

contract ZkManager is AccessControl {

    // The maintainer role is allowed to upgrade the contract.
    bytes32 public constant MAINTAINER_ROLE = keccak256("MAINTAINER_ROLE");

    // The authority role is allowed to add and remove the verifier authority.
    bytes32 public constant AUTHORITY_ROLE = keccak256("AUTHORITY_ROLE");

    // The address of the verifier contract.
    address public verifierContract;

    // The address of the contract that is allowed to upgrade the contract.
    address public upgradeAddress;

    // A counter of the number of proofs validated.
    uint256 public proofCounter;

    // The event that is emitted when the contract is upgraded.
    event ContractUpgraded(address indexed newAddress);

    // The event that is emitted when the upgrade fails.
    event UpgradeFailed(string reason);

    // The event that is emitted when the verifier contract is set.
    event VerifierUpgraded(address indexed newVerifier);

    // The event that is emitted when a proof is validated.
    event ProofValidated(bool success);

    // The event that is emitted when a proof is rejected.
    event ProofRejected(bool success);

    // The event that is emitted when a new authority is added.
    event AuthorityAdded(address indexed newAuthority);

    // The event that is emitted when an authority is removed.
    event AuthorityRemoved(address indexed removedAuthority);

    // The event that is emitted when a new maintainer is added.
    event MaintainerAdded(address indexed newMaintainer);

    // The event that is emitted when a maintainer is removed.
    event MaintainerRemoved(address indexed removedMaintainer);

    constructor(address initialVerifier) {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(MAINTAINER_ROLE, msg.sender);
        _grantRole(AUTHORITY_ROLE, msg.sender);
        verifierContract = initialVerifier;
    }

    // Modifier used to check if the caller is an admin or a specific role.
    modifier onlyAdminOrRole(bytes32 role) {
        require(hasRole(role, msg.sender) || hasRole(DEFAULT_ADMIN_ROLE, msg.sender), "Restricted to admins or specific role.");
        _;
    }

    // Modifier used to check if the contract has been upgraded.
    modifier onlyNotUpgraded() {
        require(upgradeAddress == address(0), "Contract already upgraded");
        _;
    }

    // Function used to add maintainers, which are allowed to upgrade the contract.
    function addMaintainers(address[] memory managers) public onlyAdminOrRole(DEFAULT_ADMIN_ROLE) onlyNotUpgraded() {
        for (uint i = 0; i < managers.length; i++) {
            _grantRole(MAINTAINER_ROLE, managers[i]);
            emit MaintainerAdded(managers[i]);
        }
    }

    // Function used to remove maintainers, which are allowed to upgrade the contract.
    function removeMaintainers(address[] memory managers) public onlyAdminOrRole(DEFAULT_ADMIN_ROLE) onlyNotUpgraded() {
        for (uint i = 0; i < managers.length; i++) {
            _revokeRole(MAINTAINER_ROLE, managers[i]);
            emit MaintainerRemoved(managers[i]);
        }
    }

    // Function used to upgrade the contract, which is allowed to be upgraded by maintainers.
    function upgradeContract(address newAddress) public onlyRole(MAINTAINER_ROLE) onlyNotUpgraded() {
        // First, confirm the current state of the new contract to avoid unnecessary migration
        bool currentStateCorrect = false;
        try IUpgradeable(newAddress).confirmState(proofCounter) returns (bool result) {
            currentStateCorrect = result;
        } catch {
            emit UpgradeFailed("Failed to confirm current state");
            revert("Failed to confirm current state");
        }

        // Proceed with state import only if the current state is not correct
        if (!currentStateCorrect) {
            try IUpgradeable(newAddress).importState(proofCounter) {
                // After successful import, confirm the state to ensure it's updated correctly
                bool isConfirmed = false;
                try IUpgradeable(newAddress).confirmState(proofCounter) returns (bool result) {
                    isConfirmed = result;
                } catch {
                    emit UpgradeFailed("Failed to confirm imported state");
                    revert("Failed to confirm imported state");
                }

                require(isConfirmed, "State confirmation failed after import");
            } catch {
                emit UpgradeFailed("Failed to import state");
                revert("Failed to import state");
            }
        }

        // Set the upgrade address and emit the upgraded event if all checks are passed
        upgradeAddress = newAddress;
        emit ContractUpgraded(newAddress);
    }


    // Function used to add authorities, which are allowed to upgrade the verifier contract.
    function addAuthority(address[] memory authorities) public onlyAdminOrRole(AUTHORITY_ROLE) onlyNotUpgraded() {
        for (uint i = 0; i < authorities.length; i++) {
            _grantRole(AUTHORITY_ROLE, authorities[i]);
            emit AuthorityAdded(authorities[i]);
        }
    }

    // Function used to remove authorities, which are allowed to upgrade the verifier contract.
    function removeAuthority(address[] memory authorities) public onlyAdminOrRole(AUTHORITY_ROLE) onlyNotUpgraded() {
        for (uint i = 0; i < authorities.length; i++) {
            _revokeRole(AUTHORITY_ROLE, authorities[i]);
            emit AuthorityRemoved(authorities[i]);
        }
    }

    // Function used to set the verifier contract, which is allowed to be set by authorities.
    function setVerifierContract(address newVerifier) public onlyAdminOrRole(AUTHORITY_ROLE) onlyNotUpgraded()  {
        verifierContract = newVerifier;
        emit VerifierUpgraded(newVerifier);
    }

    // Function used to batch validate proofs, which is allowed to be called by anyone.
    function batchValidateProofs(bytes[] calldata proofs) public onlyNotUpgraded() returns (bool[] memory results) {
        // Try to validate the proofs, or revert if the verifier contract call fails.
        try IRiscZeroVerifier(verifierContract).verifyProofs(proofs) returns (bool[] memory _results) {
            results = _results;
            for (uint i = 0; i < results.length; i++) {
                if (results[i]) {
                    emit ProofValidated(results[i]);
                    proofCounter++;
                } else {
                    emit ProofRejected(results[i]);
                }
            }
        } catch {
            // If the verifier contract call fails, revert.
            // Changes are not applied, and events are not emitted.
            revert("Verifier contract call failed");
        }
        return results;
    }
}
