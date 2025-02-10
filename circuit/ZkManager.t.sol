// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import "ZkManager.sol";

// Mock verifier contract for testing
contract MockVerifier {
    function verifyProofs(bytes[] calldata proofs) external pure returns (bool[] memory) {
        bool[] memory results = new bool[](proofs.length);
        for (uint i = 0; i < proofs.length; i++) {
            results[i] = proofs[i].length > 0; // Simple mock validation
        }
        return results;
    }
}

// Mock upgradeable contract for testing
contract MockUpgradeable {
    uint256 public state;

    function importState(uint256 _state) external {
        state = _state;
    }

    function confirmState(uint256 _state) external view returns (bool) {
        return state == _state;
    }
}

contract ZkManagerTest is Test {
    ZkManager validator;
    MockVerifier verifier;
    MockUpgradeable upgradeable;
    address admin;
    address maintainer;
    address authority;

    function setUp() public {
        // Set up addresses
        admin = address(this);
        maintainer = address(0x2);
        authority = address(0x3);

        // Deploy mock verifier
        verifier = new MockVerifier();
        
        // Deploy main contract with test address as admin
        validator = new ZkManager(address(verifier));

        // Deploy mock upgradeable contract
        upgradeable = new MockUpgradeable();
        
        // Add roles using admin
        address[] memory maintainers = new address[](1);
        maintainers[0] = maintainer;
        validator.addMaintainers(maintainers);

        address[] memory authorities = new address[](1);
        authorities[0] = authority;
        validator.addAuthority(authorities);
    }

    function testProofValidation() public {
        bytes[] memory proofs = new bytes[](2);
        proofs[0] = bytes("valid proof");
        proofs[1] = bytes("");

        bool[] memory results = validator.batchValidateProofs(proofs);
        
        assertTrue(results[0], "Valid proof should be accepted");
        assertFalse(results[1], "Empty proof should be rejected");
        assertEq(validator.proofCounter(), 1, "Counter should increment for valid proofs only");
    }

    function testUpgradeContract() public {
        vm.prank(maintainer);
        validator.upgradeContract(address(upgradeable));
        assertEq(validator.upgradeAddress(), address(upgradeable), "Upgrade address should be set");
    }

    function testUpgradeContractFailsForNonMaintainer() public {
        vm.prank(authority);
        vm.expectRevert();
        validator.upgradeContract(address(upgradeable));
    }

    function testSetVerifier() public {
        address newVerifier = address(new MockVerifier());
        
        vm.prank(authority);
        validator.setVerifierContract(newVerifier);
        assertEq(validator.verifierContract(), newVerifier, "Verifier should be updated");
    }

    function testSetVerifierFailsForNonAuthority() public {
        address newVerifier = address(new MockVerifier());
        
        vm.prank(maintainer);
        vm.expectRevert("Restricted to admins or specific role.");
        validator.setVerifierContract(newVerifier);
    }

    function testAddRemoveMaintainers() public {
        address newMaintainer = address(0x4);
        
        address[] memory managers = new address[](1);
        managers[0] = newMaintainer;
        
        validator.addMaintainers(managers);
        assertTrue(validator.hasRole(validator.MAINTAINER_ROLE(), newMaintainer), "Should add maintainer");
        
        validator.removeMaintainers(managers);
        assertFalse(validator.hasRole(validator.MAINTAINER_ROLE(), newMaintainer), "Should remove maintainer");
    }

    function testAddRemoveAuthorities() public {
        address newAuthority = address(0x5);
        
        address[] memory authorities = new address[](1);
        authorities[0] = newAuthority;
        
        validator.addAuthority(authorities);
        assertTrue(validator.hasRole(validator.AUTHORITY_ROLE(), newAuthority), "Should add authority");
        
        validator.removeAuthority(authorities);
        assertFalse(validator.hasRole(validator.AUTHORITY_ROLE(), newAuthority), "Should remove authority");
    }
}