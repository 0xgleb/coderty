// SPDX-License-Identifier: BSL
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/governance/Governor.sol";
import "@openzeppelin/contracts/governance/extensions/GovernorSettings.sol";
import "@openzeppelin/contracts/governance/extensions/GovernorCountingSimple.sol";
import "@openzeppelin/contracts/governance/extensions/GovernorVotes.sol";
import "@openzeppelin/contracts/governance/extensions/GovernorVotesQuorumFraction.sol";

import "./Contributions.sol";

contract Project is Governor, GovernorSettings, GovernorCountingSimple, GovernorVotes, GovernorVotesQuorumFraction {
    Contributions public trackerContract;

    bool private _initialized;
    mapping(uint256 => bytes) private _proposalIpfsHashes;
    mapping(uint256 => string) private _proposalDescriptions;

    constructor(string memory name)
        Governor(name)
        GovernorSettings(7200, /* 1 day */ 50400, /* 1 week */ 0)
        GovernorVotes(new Contributions())
        GovernorVotesQuorumFraction(66)
    {
        trackerContract = Contributions(address(token));
    }

    function initialize(bytes calldata ipfsHash) public {
        require(!_initialized, "Project: already initialized");
        _initialized = true;
        trackerContract.safeMint(msg.sender, ipfsHash);
    }

    function submitContributionsRequest(bytes calldata ipfsHash, string calldata description)
        public
        returns (uint256)
    {
        address[] memory targets = new address[](1);
        targets[0] = address(token);
        uint256[] memory values = new uint256[](1);
        values[0] = 0;
        bytes memory input = abi.encodeWithSignature("safeMint(address,bytes)", msg.sender, ipfsHash);
        bytes[] memory calldatas = new bytes[](1);
        calldatas[0] = input;
        uint256 proposalId = propose(targets, values, calldatas, description);
        _proposalIpfsHashes[proposalId] = ipfsHash;
        _proposalDescriptions[proposalId] = description;
        return proposalId;
    }

    function getProposalIpfsHash(uint256 proposalId) public view returns (bytes memory) {
        return _proposalIpfsHashes[proposalId];
    }

    function inludeContributions(uint256 proposalId) public {
        address[] memory targets = new address[](1);
        targets[0] = address(token);
        uint256[] memory values = new uint256[](1);
        values[0] = 0;
        bytes memory input =
            abi.encodeWithSignature("approveContribution(address,bytes)", msg.sender, _proposalIpfsHashes[proposalId]);
        bytes[] memory calldatas = new bytes[](1);
        calldatas[0] = input;
        propose(targets, values, calldatas, _proposalDescriptions[proposalId]);
    }

    // The following functions are overrides required by Solidity.

    function votingDelay() public view override(IGovernor, GovernorSettings) returns (uint256) {
        return super.votingDelay();
    }

    function votingPeriod() public view override(IGovernor, GovernorSettings) returns (uint256) {
        return super.votingPeriod();
    }

    function quorum(uint256 blockNumber)
        public
        view
        override(IGovernor, GovernorVotesQuorumFraction)
        returns (uint256)
    {
        return super.quorum(blockNumber);
    }

    function proposalThreshold() public view override(Governor, GovernorSettings) returns (uint256) {
        return super.proposalThreshold();
    }
}
