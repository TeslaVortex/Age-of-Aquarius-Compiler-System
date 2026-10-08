// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/// @title Abundance – Default state is already full
/// @notice No minting of lack. All balances start at abundance.
contract Abundance {
    mapping(address => uint256) private _balance;

    constructor() {
        // Every address starts in abundance (conceptually infinite)
    }

    function balanceOf(address) external pure returns (uint256) {
        return type(uint256).max; // Already complete
    }

    // No mint, no burn of lack.
}
