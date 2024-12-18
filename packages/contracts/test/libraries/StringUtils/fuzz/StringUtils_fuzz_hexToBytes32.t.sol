// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import { StringUtils } from "src/libraries/StringUtils.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { StructHelper } from "../../../helpers/StructHelper.sol";

// Currently, tests in this contract are disabled on zkSync 
// due to LLVM issues with Strings and StringUtils libraries
contract StringUtils_HexToBytes32_Fuzz_Test is StructHelper {
    using Strings for uint256;

    function setUp() public override {
        super.setUp();
    }
    
    function testFuzz_hexToBytes32_ComputesExpectedHash(bytes32 expectedHash) public {
        skipIfZkSync();

        string memory hashString = uint256(expectedHash).toHexString(32);
        bytes32 hashResult = StringUtils.hexToBytes32(hashString);

        assertEq(hashResult, expectedHash);
    }

    function testFuzz_hexToBytes32_ComputesExpectedAddressHash(address addressToHash) public {
        skipIfZkSync();

        bytes32 expectedHash = keccak256(abi.encodePacked(addressToHash));
        string memory hashString = uint256(expectedHash).toHexString(32);
        bytes32 hashResult = StringUtils.hexToBytes32(hashString);

        assertEq(hashResult, expectedHash);
    }

    function testFuzz_hexToBytes32_ComputesExpectedCalldataHash(bytes memory calldataToHash)
        public
    {
        skipIfZkSync();

        bytes32 expectedHash = keccak256(calldataToHash);
        string memory hashString = uint256(expectedHash).toHexString(32);

        bytes32 hashResult = StringUtils.hexToBytes32(hashString);

        assertEq(hashResult, expectedHash);
    }
}
