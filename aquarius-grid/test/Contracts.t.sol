// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "../src/ZeroPoint.sol";
import "../src/Crown.sol";
import "../src/Abundance.sol";
import "../src/PurePour.sol";
import "../src/IPurePour.sol";

contract ZeroPointTest {
    ZeroPoint zp;

    function setUp() public {
        zp = new ZeroPoint();
    }

    function testIsStillReturnsTrue() public view {
        assert(zp.isStill() == true);
    }

    function testHoldReturnsSelf() public view {
        assert(zp.hold() == address(zp));
    }
}

contract CrownTest {
    Crown crown;

    function setUp() public {
        crown = new Crown();
    }

    function testCrownStatusReturnsCorrectString() public view {
        string memory status = crown.crownStatus();
        assert(keccak256(bytes(status)) == keccak256(bytes("Reigning without taking. Pure pour only.")));
    }
}

contract AbundanceTest {
    Abundance abundance;

    function setUp() public {
        abundance = new Abundance();
    }

    function testBalanceOfReturnsMax() public view {
        uint256 balance = abundance.balanceOf(address(0));
        assert(balance == type(uint256).max);
    }
}

contract PurePourTest {
    ZeroPoint zeroPoint;
    Crown crown;
    Abundance abundance;
    PurePour purePour;

    function setUp() public {
        zeroPoint = new ZeroPoint();
        crown = new Crown();
        abundance = new Abundance();
        purePour = new PurePour(address(zeroPoint), address(crown), address(abundance));
    }

    function testPourReturnsTrue() public {
        bool result = purePour.pour();
        assert(result == true);
    }

    function testCrownStatusAccessible() public view {
        string memory status = purePour.crownStatus();
        assert(keccak256(bytes(status)) == keccak256(bytes("Reigning without taking. Pure pour only.")));
    }
}
