// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {console} from "forge-std/console.sol";
import "../src/utils/Verifier.sol";
import "../src/utils/ECDSAOwnedDKIMRegistry.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";
import {UserOverrideableDKIMRegistry} from "@zk-email/contracts/UserOverrideableDKIMRegistry.sol";
import {BaseDeployScript} from "./BaseDeployScript.sol";

contract Upgrades is BaseDeployScript {
    function run() public override {
        super.run();

        address verifierProxy = vm.envOr("VERIFIER", address(0));
        address ecdsaDkimProxy = vm.envOr("ECDSA_DKIM", address(0));
        address dkimProxy = vm.envOr("DKIM", address(0));

        vm.startBroadcast(deployerPrivateKey);
        if (verifierProxy != address(0)) {
            upgradeVerifier(verifierProxy);
        }
        if (ecdsaDkimProxy != address(0)) {
            upgradeECDSAOwnedDKIMRegistry(ecdsaDkimProxy);
        }
        if (dkimProxy != address(0)) {
            upgradeUserOverrideableDKIMRegistry(dkimProxy);
        }
        vm.stopBroadcast();
    }

    function upgradeVerifier(address verifierProxy) public {
        Verifier newVerifierImpl = new Verifier();
        UUPSUpgradeable(verifierProxy).upgradeToAndCall(
            address(newVerifierImpl),
            bytes("")
        );
    }

    function upgradeECDSAOwnedDKIMRegistry(address ecdsaDkimProxy) public {
        ECDSAOwnedDKIMRegistry newECDSAOwnedDKIMRegistryImpl = new ECDSAOwnedDKIMRegistry();
        UUPSUpgradeable(ecdsaDkimProxy).upgradeToAndCall(
            address(newECDSAOwnedDKIMRegistryImpl),
            bytes("")
        );
    }

    function upgradeUserOverrideableDKIMRegistry(address dkimProxy) public {
        UserOverrideableDKIMRegistry newDkimImpl = new UserOverrideableDKIMRegistry();
        UUPSUpgradeable(dkimProxy).upgradeToAndCall(
            address(newDkimImpl),
            bytes("")
        );
    }
}
