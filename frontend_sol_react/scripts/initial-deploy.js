const {ethers} = require("hardhat");

async function main() {
    // We get the contract to deploy
    const factory = await ethers.getContractFactory("UsersPublicHashes");
    // Deploying contracts
    const usersPublicHashes = await factory.deploy();
    // const memberManager = await upgrades.deployProxy(factory2);

    await usersPublicHashes.deployed();

    console.log("UsersPublicHashes deployed to: ", usersPublicHashes.address);
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });