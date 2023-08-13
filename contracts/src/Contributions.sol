// SPDX-License-Identifier: BSL
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/cryptography/EIP712.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Votes.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

contract Contributions is ERC721, Ownable, EIP712, ERC721Votes {
    using Counters for Counters.Counter;

    Counters.Counter private _tokenIdCounter;

    // Mapping from token ID to git patch IPFS hash
    mapping(uint256 => bytes) private _patches;
    mapping(uint256 => string) private _descriptions;

    constructor() ERC721("Contributions", "CNTR") EIP712("Contributions", "1") {}

    function safeMint(address to, bytes calldata ipfsHash) public onlyOwner {
        uint256 tokenId = _tokenIdCounter.current();
        _tokenIdCounter.increment();
        _patches[tokenId] = ipfsHash;
        _safeMint(to, tokenId);
    }

    function patchCount() public view returns (uint256) {
        return _tokenIdCounter.current();
    }

    function getPatch(uint256 tokenId) public view returns (bytes memory) {
        return _patches[tokenId];
    }

    function getDescription(uint256 tokenId) public view returns (string memory) {
        return _descriptions[tokenId];
    }

    // The following functions are overrides required by Solidity.

    function _afterTokenTransfer(address from, address to, uint256 tokenId, uint256 batchSize)
        internal
        override(ERC721, ERC721Votes)
    {
        super._afterTokenTransfer(from, to, tokenId, batchSize);
    }
}
