// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/Project.sol";
import "../src/Contributions.sol";

contract ContributionsTest is Test {
    Project public project;

    function setUp() public {
        project = new Project("Test Project");
    }

    function testHappyPath() public {
        // 1. initialize
        // 2. submit proposal
        // 3. vote
        // 4. include contributions

        project.initialize(bytes("QmTPuu4vApDLxzbXzJCCLpvdi6DNComvGzwZrca38HjhKZ"));

        uint256 proposalId =
            project.submitContributionsRequest(bytes("QmTPuu4vApDLxzbXzJCCLpvdi6DNComvGzwZrca38HjhKZ"), "Test Proposal");

        project.castVote(proposalId, type(uint8).max);

        project.inludeContributions(proposalId);
    }
}
