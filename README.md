# About
Zero-Knowledge Password Manager is a Proof of Concept project, which shows how an on-chain password manager could work with the use of Zero-Knowledge Protocols (ZKP)

# Technical part
The backend part of the application is built in Rust with the use arkworks ecosystem and actix.
1. [Arkworks ecosystem](https://github.com/arkworks-rs) is a set of Rust crates needed for Zero-Knowledge implementation. It is also worth noting, that arkworks is still in the active development and must not be used for in production!
2. [Actix](https://actix.rs/) is a framework used for web development. It was used to connect to the frontend part of the application.

Web side of the application consists of a React project and a single smart contract, written in Solidity.
1. React application is mainly done to help users to interact with Rust implementation.
2. The mentioned smart contract (UsersPublicHashes.sol) is used to store the Public hashes of the users, which are essential for further user verification, when they are trying to "log in".

More technical information will be added to the document and attached to this repository later.

# How to use the project

## Prerequisites 
Firstly, you will need some initial setup:
1. Download the latest [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) version.
2. Download the latest [npm](https://nodejs.org/en/download/) version.
3. Create a wallet in MetaMask, add MetaMask as a browser extension and login there. ([source](https://www.geeksforgeeks.org/how-to-install-and-use-metamask-on-google-chrome/))
4. Add Avalanche testnet to networks in MetaMask: ([source](https://support.avax.network/en/articles/6224787-how-to-connect-to-the-fuji-testnet))
5. Add some test funds to the newly created account: ([source](https://core.app/tools/testnet-faucet))

## Worth mentioning
The contract ABI and the contract address in function getContract() are from already deployed and "used" contract. It will perfectly work on your system, but if you decide to deploy a new one, you will need to change "avaxTestnetAccountPrivateKey" in hardhat.config.js to the private key of your account, which has faucets in the specified earlier testnet.

Then you will need to run the command `npx hardhat run --network avaxTestnet scripts/initial-deploy.js` in the console, in the root folder.

Then change the contract address in getContract() method, to the one you will see in the console. And if you changed the smart contract code before deployment change the ABI from the "utils" folder to the one from the automatically generated "artifacts/contracts/UsersPublicHashes.sol/UsersPublicHashes.json".

Summarizing, there will be four following steps if you decide to redeploy the smart contract:
1. Change "avaxTestnetAccountPrivateKey" in hardhat.config.js file to your account private key with avax testnet assets.
2. Run `npx hardhat run --network avaxTestnet scripts/initial-deploy.js` in the console.
3. Change contract address in getContract() to the new one, which you will see on the console.
4. Change the old contract ABI from "utils" to the new one in "artifacts/contracts/UsersPublicHashes.sol/UsersPublicHashes.json".

## Usage

1. In the Rust part of the project, run `cargo run` command in the root of the folder.
2. In the React project root run the following commands:
   1. Firstly, run `npm install`.
   2. Then run `npm start`.
3. If the tab didn't open automatically, go to the `localhost:3000` in the browser (Note: it is usually `localhost:3000`, but you may also have `localhost:8080` instead).

# More detailed information

For more detailed information about the project and how it works, check the Proof_of_Concept.md file.