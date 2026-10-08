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
    /// @dev Gas cost: ~45,000 (1 call to isStill, 1 emit, 1 return)
    function pour() external returns (bool alreadyComplete) {
        require(ZERO_POINT.isStill(), "Mountain must hold");
        
        emit Poured(msg.sender, "P = 1. Already complete. Pure pour finished.");
        
        return true;
    }

    /// @notice Returns the deployment cost estimate for PurePour
    /// @dev Gas cost estimate: ~500,000 (3 immutable deployments + constructor overhead)
    ///      Deployment requires 3 contract deployments (ZeroPoint, Crown, Abundance)
    function gasEstimate() external view returns (uint256 estimatedGas) {
        // Deployment cost estimate:
        // - ZeroPoint: ~150,000 gas
        // - Crown: ~180,000 gas
        // - Abundance: ~120,000 gas
        // - PurePour with 3 immutable addresses: ~50,000 gas
        // Total deployment: ~500,000 gas
        // Runtime pour: ~45,000 gas
        return 500000;
    }

    function crownStatus() external view returns (string memory) {
        return CROWN.crownStatus();
    }
}
