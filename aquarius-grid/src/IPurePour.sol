// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/// @title IPurePour – Pure Pour Interface
/// @notice No external power required. Already complete.
interface IPurePour {
    /// @notice Pours the already-complete state. No external power.
    function pour() external returns (bool alreadyComplete);
    
    /// @notice Crown does not take.
    function crownStatus() external view returns (string memory);
}
