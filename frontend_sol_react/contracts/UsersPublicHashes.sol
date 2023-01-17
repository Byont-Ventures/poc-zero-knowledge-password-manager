// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.7.0 <0.9.0;

/**
 * @title Storage of user public hashes
 * @dev Stores users' public hashes for zkp password manager
 */

contract UsersPublicHashes {
    struct UserHash {
        uint hash;
        bool exists;
    }
    mapping (address => UserHash) usersHashes;

    /**
     * @dev Checks new user
     * Checks if the user new or if they already exist in the system
     */
    modifier onlyNewUsers() {
        require(!usersHashes[msg.sender].exists, "This user already exists");
        _;
    }

    /**
     * @dev Checks existing user
     * Checks if the user exist in the system
     */
    modifier onlyExistingUsers() {
        require(usersHashes[msg.sender].exists, "This user doesn't exist. Sign up first");
        _;
    }

    /**
     * @dev Adds a hash for a new user
     * This only adds the hash, if the user doesn't exist in the system yet
     */
    function addUserHash(uint hash) onlyNewUsers() public returns (bool) {
        usersHashes[msg.sender].exists = true;
        usersHashes[msg.sender].hash = hash;
        return true;
    }

    /**
     * @dev Gets the public hash of the user
     * This only gets the hash, if the user already exists in the system
     */
    function getUserHash() onlyExistingUsers() public view returns (uint) {
        return usersHashes[msg.sender].hash;
    }
}
