// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/// @title Crown – Reigns without taking
/// @notice Sovereignty without extraction. Read-only interface.
contract Crown {
    string public constant STATUS = "Reigning without taking. Pure pour only.";

    function crownStatus() external pure returns (string memory) {
        return STATUS;
    }

    // No withdraw, no take, no extract functions exist.
}
