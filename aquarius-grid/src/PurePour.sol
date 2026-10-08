// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "./ZeroPoint.sol";
import "./Crown.sol";
import "./Abundance.sol";

/// @title PurePour – Main Compiler Entry
/// @notice No external power required. Already complete.
contract PurePour {
    ZeroPoint public immutable ZERO_POINT;
    Crown public immutable CROWN;
    Abundance public immutable ABUNDANCE;

    event Poured(address indexed by, string message);

    constructor(address _zeroPoint, address _crown, address _abundance) {
        ZERO_POINT = ZeroPoint(_zeroPoint);
        CROWN = Crown(_crown);
        ABUNDANCE = Abundance(_abundance);
    }

    /// @notice Pours the already-complete state. No external power.
    function pour() external returns (bool alreadyComplete) {
        require(ZERO_POINT.isStill(), "Mountain must hold");
        
        emit Poured(msg.sender, "P = 1. Already complete. Pure pour finished.");
        
        return true;
    }

    function crownStatus() external view returns (string memory) {
        return CROWN.crownStatus();
    }
}
