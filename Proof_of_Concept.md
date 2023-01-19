# Proof of Concept

# Table of Contents

<!-- TOC -->
* [Introduction](#introduction)
* [Prerequisites](#prerequisites)
* [Idea description](#idea-description)
  * [How it works](#how-it-works)
     * [Definitions](#definitions)
     * [Public hash generation](#public-hash-generation)
     * [Zero-Knowledge verification](#zero-knowledge-verification)
  * [Potential advantages](#potential-advantages)
    * [Rainbow table resistance](#rainbow-table-resistance)
    * [Random final output](#random-final-output)
    * [API features](#api-features)
  * [Potential vulnerabilities](#potential-vulnerabilities)
    * [Brute-force attacks](#brute-force-attacks)
    * [Toxic waste exploit](#toxic-waste-exploit)
  * [Possible further improvements](#possible-further-improvements)
    * [Public hash generation](#public-hash-generation)
    * [Limit password checks](#limit-password-checks)
    * [Shnorr signature](#shnorr-signature)
* [Details](#details)
  * [Arkworks](#arkworks)
  * [Groth16](#groth16)
* [Anonymous Voting](#anonymous-voting)
  * [Introduction](#introduction)
  * [Idea 1](#idea-1)
    * [Description](#description)
    * [Open issues](#open-issues)
  * [Idea 2](#idea-2)
    * [Short description](#short-description)
    * [Full description](#full-description)
    * [Further ideas](#further-ideas)
  * [Idea 3](#idea-3)
    * [Description](#description)
<!-- TOC -->

# Introduction

This document shows the details about Proof of Concept: the idea, how it works and how it is implemented. Moreover, the previous Proof of Concept ideas are also mentioned here in case anyone finds them interesting and/or decides to develop them.

It is also worth mentioning that this project is not stable and is not production ready (yet). Moreover, the ***arkworks*** repositories, used for Zero-Knowledge Proof usage should not be used in production either.

# Prerequisites

This document assumes that the reader is knowledgeable in the fields of Zero-Knowledge Proofs, Pedersen Commitments and Elliptic Curves. If you find yourself not understanding certain concepts while reading this document, it is advised to read the following articles:

1. ****[Zero Knowledge Proof - How it works and The Alibaba Cave Experiment](https://www.byont.io/blog/zero-knowledge-proof-how-it-works-and-the-alibaba-cave-experiment)****
2. ****[Zero-Knowledge Proof - Types, Protocols, and Implementations used in Blockchain](https://www.byont.io/blog/zero-knowledge-proof-types-protocols-and-implementations-used-in-blockchain)****
3. ****[Zero-Knowledge Proof - Cryptographic Primitives and Sigma Protocol](https://www.byont.io/blog/zero-knowledge-proof-cryptographic-primitives-and-sigma-protocol)****

# Idea description

Zero-Knowledge Password Manager is a password manager that utilizes Zero-Knowledge Proofs and that can be used in web3 environment. Additionally, this password manager implements a new vision of password hiding, which will be explained further.

## How it works

Zero-Knowledge Password Manager has two main operations: public hash generation and zero-knowledge verification. But these parts will be explained after defining, how main variables will be called.

### Definitions

There are three main variables in both generation and verification steps: password, wallet and hash.

- Password ( $password$ ) — a password, that a user sets and verifies, when asked. This is a secret value that should be known only to a its owner.
- Wallet ( $wallet$ ) — an address of a user’s crypto wallet account (single wallet can have multiple accounts and thus multiple addresses). This value is public and unique for every account and every user.
- Hash ( $hash$ ) — (sometimes can be referred to as “*public hash*”) is a hash, which is calculated in the Public hash generation, stored in a smart contract and then used in the password verification.

At some point password is separated into two parts: pass and word.

- Pass ( $pass$ ) — the first part of the password.
- Word ( $word$ ) — the second part of the password.

### Public hash generation

1. *(Prerequisite)* The user has to connect their wallet to the website, otherwise the public generation will fail.
2. Firstly, the user $password$ is hashed.
3. Then this hash is separated into two parts: $pass$ and $word$.
4. The $pass$  and $word$ are also hashed.
5. Although, the $wallet$ is already a hash, it is hashed to make it similar to other hashes.
6. Now the $hash$ value is calculated using the following equation: $wallet * pass = hash + word$.
7. The calculated $hash$ value is then submitted to the smart contract, which has a mapping that keeps track of the user $wallet$s and user calculated $hash$es. 

### Zero-Knowledge verification

1. *(Prerequisite)* The user has to connect their wallet to the website, otherwise the verification will fail.
2. Firstly, the $hash$ is retrieved from the smart contract, according to the user $wallet$.
3. Then the user $password$ is hashed.
4. Then this hash is separated into two parts: $pass$ and $word$.
5. The $pass$  and $word$ are also hashed.
6. In the end we submit these values, do the ***arkworks*** magic and get a *bool* as a response: *true* if the password is correct, *false* if it is not. 
    1. If you want to learn, how the password is verified using ***arkworks*** (which utilizes *zk-SNARK* logic), check the *Details/Arkworks* section.

## Potential advantages

### Rainbow table resistance

The public hash, that is stored on a smart contract, is never the same for any pair of users. This is because the public hash calculation is dependent on the wallet address of the user, which is unique. This means that if two users have the same password, their public hashes will be different. 

### Random final output

***Prerequisite:*** check the Details/Arkworks before reading further for better understanding. 

When verifying a password with ***arkworks*** in the end there are two Pedersen commitments that are compared with each other $commitment_a == commitment_b$, where $commitment_a = C(hash)$ and $commitment_b = C(wallet * pass - word)$. Here $C()$ is a function that converts an input into a corresponding Pedersen commitment. Let’s then say, that the final commitments should be equal to a particular final value $final = commitment_a = commitment_b$. It would be easier to hack the password, if the hacker knew this $final$ value. However, when calculating commitments a random value is always used, which means that during every new verification the same input: $hash$, $wallet$ and $password$ will produce different commitments $commitment_{a-1}, commitment_{a-2}, commitment_{b-1}, commitment_{b-2}$, where $commitment_{a_1} ~!= commitment_{a_2}$ and $commitment_{b-1} ~!= commitment_{b-2}$, whilst $commitment_{a-1} == commitment_{b-1}$ and $commitment_{a-2} == commitment_{b-2}$. Which also means that the corresponding final values $final_1$ and $final_2$ will never be equal.

### API features

The backend part of the application, which is responsible for Public hash generation and for password verification with ZKP, acts as an API. The smart contract that stores public hashes in it, is shown only as an example. It can be easily substituted with any other smart contract, that has similar mapping. This means that all the functionality of this application can be easily inherited by other web3 projects and organizations.

## Potential vulnerabilities

### Brute-force attacks

A hostile user can still find the right password via trial-and-error method, however it is less obvious to achieve, because this password manager is resistant to the rainbow table attacks, as mentioned earlier.

### Toxic waste exploit

Groth16 due to its setup phase produces the Toxic Waste, which later can be used by a user to create fake proofs that will pass the ZKP check. (To learn more about the toxic waste, check the [source](https://medium.com/qed-it/how-toxic-is-the-waste-in-a-zksnark-trusted-setup-9b250d59bdb4))

Note, this vulnerability is only theoretical. It is most likely, that it would be impossible to use toxic waste for the user’s advantage, as the ZK verification happens locally and one go (in a single method). However, the possibility of this vulnerability should be taken into consideration.   

## Possible further improvements

### Public hash generation

Currently the public hash is generated using the next equation: $wallet * pass = hash + word$. It is not the most secure generation. I believe, that the best way to compute the hash would be with using discrete logarithm problem: $wallet^{pass} = hash^{word}$. Because of time constraints, I didn’t find a way to implement this equation, but I believe it is possible.

### Limit password checks

The users would be able to verify their password only several times per time period. That may reduce a possibility of a single user being a victim of a brute-force attack.

### Shnorr signature

Shnorr signatures can be used to verify that the user that is trying to verify the password is an account owner. It can be used in a way that the public hash can be accessed only when its owner is trying to pass a verification.

Personally I believe that combination of this password manager and the Shnorr signature can be also seen as a web3 version of the two-step verification.

# Details

## Arkworks

This is how the password verification happens in the ***arkworks*** section after receiving the values $hash$, $pass$, $word$, $wallet$.

It will be separated into two smaller parts: *Circuit* and *Groth16*, to make it easier to understand the workflow of Zero-Knowledge usage.

1. However, first of all, a pedersen commitment ( $commitment_a$ ) is created with a $hash$ as an input. This happens before any *Circuit* or *Groth16* logic.

### Circuit

*Circuit* section explains, what happens in the circuit, that is sent to the Zero-Knowledge setup to verify the password. In this context, you can see a *“circuit”* as a function that defines the verification code that should be executed and that defines *private variables (witnesses)*, *constants* and *public inputs* for the Zero-Knowledge Proof setup.

1. All three values: $pass$, $word$ and $wallet$ are declared as *witnesses.*
2. Then we calculate the $hash_2$ value from the following equation: $wallet * pass = hash + word$, which means that the $hash_2$ value is equal to $hash_2 = wallet * pass - word$.
3. After that the $hash_2$ value is used to create another Pedersen commitment: $commitmnet_b$.
4. In the end the circuit checks the validity of the next expression: $commitment_a == commitment_b$.

### Groth16

This section shows how the Zero-Knowledge setup is constructed and how it works in this application. 

1. First of all a circuit with dummy data is created. (reason for dummy data is explained later)
2. Then this circuit is supplied to the Groth16 setup function, which returns a proving key ($pk$) and a verification key ($vk$).
    1. It is needed to supply a circuit for the setup, because $pk$ and $vk$ are also connected to a particular circuit structure. It means that if you have a different circuit and $pk_2$ and $vk_2$ generated by supplying this circuit, when you will try to use a pair $pk$ and $vk_2$ or a pair $pk_2$ and $vk$, you will get an Error and the verification will fail.
    2. Also we provide dummy data here, because in this step the provided data is irrelevant. The $pk$ and $vk$ are only bonded with the structure of the circuit, the data is not taking into consideration during the bonding process. The only requirement from the data is to be correct (meaning that it should pass the checks in the circuit) or otherwise the Error will be thrown from the circuit itself.
3. After key generation, the circuit with real data is created.
4. This circuit and the proving key, $pk$, are used to generate a proof.
    1. Note, that in this step the circuit logic with all the provided data is checked before the proof generation. If it fails, an error is thrown and the program doesn’t move any further.
5. The generated proof, verification key $vk$ and the public inputs are provided to the verification method. The method returns *bool*. If it is true, then the user successfully passed ZK check, if it’s false, then it is considered that the user doesn’t know the password.
    1. As public inputs, the coordinates of the Pedersen commitment $commitment_a$ are provided, as a Pedersen commitment can also be interpreted as a point on an elliptic curve.

# Anonymous Voting

## Introduction

The Anonymous Voting project was initially supposed to be this Proof of Concept. However, due to lack of time it was decided to switch to the Zero-Knowledge Password Manager. The Anonymous Voting section will present the main ideas behind this project for those who is curious or for those who decides to develop this idea.

It is worth mentioning that further there will not be anything about Zero-Knowledge Password Manager, this section is dedicated specifically to Anonymous Voting.

## Idea 1

### Description

It is going to be some sort a whole application/website for that. Every account is going to have a private and public keys (1 public key and 1 private key per account). *It is worth mentioning that DAOs are also accounts in this case.* Then every time a person is voting on the app, their value is encrypted with Pedersen commitment and Twisted ElGamal Encryption and sent to the DAO contract. That way everyone will be able to see that the person voted but nobody will no the exact value or which option the person was voting for.

May also have Merkle Trees that will store accounts for every DAO, which means that there will also be a Merkle Tree per DAO.

### Open issues

1. How to securely store the private key for the DAO contract? 
    1. Maybe I could generate the private key every time?  But how then could I define people who already voted, if their private key will always be different (as well as the public key then, obviously).
    2. Actually, the smart contracts (or only wallets, actually) already have both private and public keys. So, maybe, that will be enough. Or will I not be able to use these values?

## Idea 2

### Short description

There are two main parties: a User and a DAO. A DAO creates a poll, Users vote there and the final result of the poll is made public, but the voters themselves are untraceable.

### Full description

1. The user creates a response / an answer, which is then encrypted with Pedersen Commitments (to make it infeasible to brute-force the answer, that the User provided).

<aside>
💡 The answer can also be verified with the ZKP after the commitment is created to make sure that this answer is legitimate (but is there even a point in this?)

</aside>

1. The answer (the commitment) is sent to the DAO party, that checks the answer and adds it to the poll. But the DAO should not be able to know, who sent the message.

<aside>
💡 As an idea, I can have a list of people that are a part of a DAO (a list of wallet addresses) and then with ZKP verify that the person, who sent an answer to the poll is a part of the DAO, but who he actually is will not be revealed.

</aside>

1. After submitting the answer the user will get a (Shnorr) signature (somehow, could be in a manifestation of NFT or POAP). Shnorr signature solves two issues at a time:
    1. The user will not be able to vote multiple times.
    2. Everyone will be able to verify that the person has voted for a specific poll. 
2. The poll itself should be hidden for everyone, so that no one could track who voted what. If possible, it will be done on-chain. Otherwise, the poll will stay off-chain until the end of the poll.

### Further ideas

1. Instead of sending number of the option in the questionary/poll send a Pedersen Commitment. (Or mb even hashed Pedersen Commitment, as a simple Pedersen Commitment could be easily checked (or not?) with arkworks). It will be sent to the Smart Contract and stored there (so that it was verifiable) and then this answer will be sent to backend, to learn what the person voted for and then send this vote to the DAO Smart Contract. 

## Idea 3

### Description

This idea involves usage of Merkle Trees. A Poll will be another entity, additionally to User and DAO. A Poll will be a Merkle Tree of all Users in the DAO. And every leaf of the tree will contain not only a user, but also their answer. It would work the next way:

1. A User submits an answer (could be possible encrypted with Pedersen Commitments, if needed for the security reasons).
2. The answer is submitted to the Poll and verified by ZKP. As inputs, there will be an initial root of the Merkle tree (Poll) and the final root of the tree. The answer of the User would be the witness there. 
3. If all the checks are passed, the new root of the Poll will be added to the DAO smart contract and the old root of the Poll will be deleted. (Potentially, the root will be submitted only in the end to make it more secure (as otherwise others will be able to see when somebody voted, although they still will no know the answer of the User, so maybe it’s not that relevant)). 
4. The Polls will be stored off-chain. The results of the poll will be shown only in the end of the poll and will not be visible while the voting is going on. 
5. Moreover, maybe, thanks to the usage of the Merkle Trees it will be possible to verify that a User is a part of a DAO by the membership verification. So, the Merkle Tree usage would kill two birds with one stone.