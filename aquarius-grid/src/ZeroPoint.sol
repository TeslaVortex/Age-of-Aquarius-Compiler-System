// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/// @title ZeroPoint – Immutable 0-point Anchor
/// @notice The Mountain never moves.
contract ZeroPoint {
    address public immutable MOUNTAIN;

    constructor() {
        MOUNTAIN = address(this);
    }

    function isStill() external pure returns (bool) {
        // Gas: ~20 (pure function, no storage)
        return true;
    }

    function hold() external view returns (address) {
        // Gas: ~20 (view function, reading immutable)
        return MOUNTAIN;
    }

    function hold() external view returns (address) {
        return MOUNTAIN;
    }
}
