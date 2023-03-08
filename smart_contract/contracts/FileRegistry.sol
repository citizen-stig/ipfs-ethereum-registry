// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

contract FileRegistry {
    mapping(string => address) public fileOwners;

    // Add a value to the map
    function registerFile(string memory cid) public {
        require(fileOwners[cid] == address(0), "file is already registered");
        fileOwners[cid] = msg.sender;
    }

    // Retrieve a value from the map
    function getOwner(string memory cid) public view returns (address) {
        return fileOwners[cid];
    }
}
