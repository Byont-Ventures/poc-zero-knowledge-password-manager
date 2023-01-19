# Proof of Concept

# Table of Contents

<!-- TOC -->
* [Introduction](#introduction)
* [0 Prerequisites](#0-prerequisites)
* [1 Idea description](#1-idea-description)
  * [1.1 How it works](#11-how-it-works)
     * [1.1.1 Definitions](#111-definitions)
     * [1.1.2 Public hash generation](#112-public-hash-generation)
     * [1.1.3 Zero-Knowledge verification](#113-zero-knowledge-verification)
  * [1.2 Potential advantages](#12-potential-advantages)
    * [1.2.1 Rainbow table resistance](#121-rainbow-table-resistance)
    * [1.2.2 Random final output](#122-random-final-output)
    * [1.2.3 API features](#123-api-features)
  * [1.3 Potential vulnerabilities](#13-potential-vulnerabilities)
    * [1.3.1 Brute-force attacks](#131-brute-force-attacks)
    * [1.3.2 Toxic waste exploit](#132-toxic-waste-exploit)
  * [1.4 Possible further improvements](#14-possible-further-improvements)
    * [1.4.1 Public hash generation](#141-public-hash-generation)
    * [1.4.2 Limit password checks](#142-limit-password-checks)
    * [1.4.3 Shnorr signature](#143-shnorr-signature)
* [2 Details](#2-details)
  * [2.1 Arkworks](#21-arkworks)
    * [2.1.1 Circuit](#211-circuit)
    * [2.1.2 Groth16](#212-groth16)
* [3 Anonymous Voting](#3-anonymous-voting)
  * [3.1 Introduction](#31-introduction)
  * [3.2 Idea 1](#32-idea-1)
    * [3.2.1 Description](#321-description)
    * [3.2.2 Open issues](#322-open-issues)
  * [3.3 Idea 2](#33-idea-2)
    * [3.3.1 Short description](#331-short-description)
    * [3.3.2 Full description](#332-full-description)
    * [3.3.3 Further ideas](#333-further-ideas)
  * [3.4 Idea 3](#34-idea-3)
    * [3.4.1 Description](#341-description)
<!-- TOC -->

# Introduction

This document shows the details of Proof of Concept: the idea, how it works, and how it is implemented. Moreover, the previous Proof of Concept ideas are also mentioned here in case anyone finds them attractive and/or decides to develop them.

It is also worth mentioning that this project is not stable and is not production ready (yet). Moreover, the ***arkworks*** repositories used for Zero-Knowledge Proof usage should not be used in production either.

# 0 Prerequisites

This document assumes that the reader knows about Zero-Knowledge Proofs, Pedersen Commitments, and Elliptic Curves. If you find yourself not understanding certain concepts while reading this document, it is advised to read the following articles:

1. **[Zero Knowledge Proof - How it works and The Alibaba Cave Experiment](https://www.byont.io/blog/zero-knowledge-proof-how-it-works-and-the-alibaba-cave-experiment)**
2. **[Zero-Knowledge Proof - Types, Protocols, and Implementations used in Blockchain](https://www.byont.io/blog/zero-knowledge-proof-types-protocols-and-implementations-used-in-blockchain)**
3. **[Zero-Knowledge Proof - Cryptographic Primitives and Sigma Protocol](https://www.byont.io/blog/zero-knowledge-proof-cryptographic-primitives-and-sigma-protocol)**

# 1 Idea description

Zero-Knowledge Password Manager is a password manager that utilizes Zero-Knowledge Proofs and that can be used in a web3 environment. Additionally, this password manager implements a new vision of password hiding, which will be explained further.

## 1.1 How it works

Zero-Knowledge Password Manager has two primary operations: public hash generation and zero-knowledge verification. But these parts will be explained after defining what main variables will be called.

### 1.1.1 Definitions

There are three main variables in the generation and verification steps: password, wallet, and hash.

- Password ( $password$ ) — a password that a user sets and verifies when required. This is a secret value that should be known only to its owner.
- Wallet ( $wallet$ ) — an address of a user’s crypto wallet account (a single wallet can have multiple accounts and thus multiple addresses). This value is public and unique for every account and every user.
- Hash ( $hash$ ) — (sometimes can be referred to as “*public hash*”) is a hash that is calculated in the Public hash generation, stored in a smart contract, and then used in the password verification.

At some point, the password is separated into two parts: pass and word.

- Pass ( $pass$ ) — the first part of the password.
- Word ( $word$ ) — the second part of the password.

### 1.1.2 Public hash generation

1. *(Prerequisite)* The user has to connect their wallet to the website. Otherwise, the public generation will fail.
2. Firstly, the user $password$ is hashed.
3. Then this hash is separated into two parts: $pass$ and $word$.
4. The $pass$ and $word$ are also hashed.
5. Although the $wallet$ is already a hash, it is hashed to make it similar to other hashes.
6. The $hash$ value is calculated using the following equation: $wallet * pass = hash + word$.
7. The calculated $hash$ value is then submitted to the smart contract, which has a mapping that keeps track of the user $wallet$s and user calculated $hash$es.

### 1.1.3 Zero-Knowledge verification

1. *(Prerequisite)* The user has to connect their wallet to the website. Otherwise, the verification will fail.
2. Firstly, the $hash$ is retrieved from the smart contract, according to the user $wallet$.
3. Then the user $password$ is hashed.
4. Then this hash is separated into two parts: $pass$ and $word$.
5. The $pass$ and $word$ are also hashed.
6. In the end, we submit these values, do the ***arkworks*** magic and get a *bool* as a response: *true* if the password is correct, *false* if it is not.
    1. If you want to learn how the password is verified using ***arkworks*** (which utilizes *zk-SNARK* logic), check the *Details/Arkworks* section.

## 1.2 Potential advantages

### 1.2.1 Rainbow table resistance

The public hash stored on a smart contract is never the same for any pair of users. This is because the public hash calculation depends on the user’s wallet address, which is unique. If two users have the same password, their public hashes will differ.

### 1.2.2 Random final output

***Prerequisite:*** Check the Details/Arkworks before reading further for better understanding.

When verifying a password with ***arkworks*** in the end, two Pedersen commitments are compared with each other $commitment_a == commitment_b$, where $commitment_a = C(hash)$ and $commitment_b = C(wallet * pass - word)$. Here $C()$ is a function that converts an input into a corresponding Pedersen commitment. Let’s say the final commitments should equal a particular final value $final = commitment_a = commitment_b$. It would be easier to hack the password if the hacker knew this $final$ value. However, when calculating commitments, a random value is always used, which means that during every new verification, the same input: $hash$, $wallet$, and $password$ will produce different commitments $commitment_{a_1}, commitment_{a_2}, commitment_{b_1}, commitment_{b_2}$, where $commitment_{a_1} ~!= commitment_{a_2}$ and $commitment_{b_1} ~!= commitment_{b_2}$, while $commitment_{a_1} == commitment_{b_1}$ and $commitment_{a_2} == commitment_{b_2}$. This also means that the corresponding final values $final_1$ and $final_2$ will never be equal.

### 1.2.3 API features

The backend part of the application, which is responsible for Public hash generation and password verification with ZKP, acts as an API. The smart contract that stores public hashes in it is shown only as an example. It can be easily substituted with any other smart contract with similar mapping. Other web3 projects and organizations can easily inherit this application’s functionality.

## 1.3 Potential vulnerabilities

### 1.3.1 Brute-force attacks

A hostile user can still find the correct password via the trial-and-error method. However, it is less obvious to achieve because this password manager is resistant to the rainbow table attacks, as mentioned earlier.

### 1.3.2 Toxic waste exploit

Groth16, due to its setup phase, produces Toxic waste, which later can be used by a user to create fake proofs that will pass the ZKP check. (To learn more about toxic waste, check the [source](https://medium.com/qed-it/how-toxic-is-the-waste-in-a-zksnark-trusted-setup-9b250d59bdb4))

Note this vulnerability is only theoretical. It is most likely that it would be impossible to use toxic waste for the user’s advantage, as the ZK verification happens locally and in one go (in a single method). However, the possibility of this vulnerability should be taken into consideration.

## 1.4 Possible further improvements

### 1.4.1 Public hash generation

Currently, the public hash is generated using the following equation: $wallet * pass = hash + word$. It is not the most secure generation. The best way to compute the hash would be by using a discrete logarithm problem: $wallet^{pass} = hash^{word}$. Because of time constraints, I didn’t find a way to implement this equation, but I believe it is possible.

### 1.4.2 Limit password checks

The users could verify their password only several times per specific time period. That may reduce a possibility of a single user being a victim of a brute-force attack.

### 1.4.3 Shnorr signature

Shnorr signatures can be used to verify that the user trying to prove the password is an account owner. It can be used to access the public hash only when its owner is trying to pass a verification.

I believe that combination of this password manager and the Shnorr signature can also be seen as a web3 version of the two-step verification.

# 2 Details

## 2.1 Arkworks

This is how the password verification happens in the ***arkworks*** section after receiving the values $hash$, $pass$, $word$, and $wallet$.

It will be separated into two smaller parts, Circuit and *Groth16*, to make understanding the workflow of Zero-Knowledge usage easier.

1. However, a Pedersen commitment ( $commitment_a$ ) is created with a $hash$ as an input. This happens before any *Circuit* or *Groth16* logic.

### 2.1.1 Circuit

*The Circuit* section explains, what happens in the circuit, that is sent to the Zero-Knowledge set up to verify the password. In this context, you can see a *“circuit”* as a function that defines the verification code that should be executed and that defines *private variables (witnesses)*, *constants*, and *public inputs* for the Zero-Knowledge Proof setup.

1. All three values: $pass$, $word$, and $wallet$ are declared as *witnesses.*
2. Then we calculate the $hash_2$ value from the following equation: $wallet * pass = hash + word$, which means that the $hash_2$ value is equal to $hash_2 = wallet * pass - word$.
3. After that, the $hash_2$ value is used to create another Pedersen commitment: $commitmnet_b$.
4. In the end, the circuit checks the validity of the following expression: $commitment_a == commitment_b$.

### 2.1.2 Groth16

This section shows how the Zero-Knowledge setup is constructed and how it works in this application.

1. First of all, a circuit with dummy data is created. (reason for dummy data is explained later)
2. Then this circuit is supplied to the Groth16 setup function, which returns a proving key ($pk$) and a verification key ($vk$).
    1. It is needed to supply a circuit for the setup because $pk$ and $vk$ are also connected to a particular circuit structure. It means that if you have a different circuit and $pk_2$ and $vk_2$ generated by supplying this circuit, when you will try to use a pair $pk$ and $vk_2$ or a pair $pk_2$ and $vk$, you will get an Error and the verification will fail.
    2. Also, we provide dummy data here because the provided data is irrelevant in this step. The $pk$ and $vk$ are only bonded with the structure of the circuit, and the data is not taken into consideration during the bonding process. The only requirement from the data is to be correct (meaning that it should pass the checks in the circuit); otherwise, the Error will be thrown from the circuit itself.
3. After key generation, the circuit with actual data is created.
4. This circuit and the proving key, $pk$, are used to generate proof.
    1. Note that in this step, the circuit logic with all the provided data is checked before the proof generation. If it fails, an error is thrown, and the program doesn’t move further.
5. The generated proof, verification key $vk$, and the public inputs are provided to the verification method. The method returns *a bool*. If it is true, then the user successfully passed the ZK check. If it’s false, then it is considered that the user doesn’t know the password.
    1. As public inputs, the coordinates of the Pedersen commitment $commitment_a$ are provided, as a Pedersen commitment can also be interpreted as a point on an elliptic curve.

# 3 Anonymous Voting

## 3.1 Introduction

The Anonymous Voting project was initially supposed to be this Proof of Concept. However, due to a lack of time, it was decided to switch to the Zero-Knowledge Password Manager. The Anonymous Voting section will present the main ideas behind this project for those curious or who choose to develop this idea.

Further, there will be nothing about Zero-Knowledge Password Manager. This section is dedicated specifically to Anonymous Voting.

## 3.2 Idea 1

### 3.2.1 Description

It is going to be a whole application/website for that. Every account will have private and public keys (1 public key and 1 private key per account). *It is worth mentioning that DAOs are also accounts in this case.* Then every time a person votes on the app, their value is encrypted with Pedersen commitment and Twisted ElGamal Encryption and sent to the DAO contract. That way, everyone can see that the person voted, but nobody will know the exact value or which option the person was voting for.

We may also have Merkle Trees that will store accounts for every DAO, which means there will also be a Merkle Tree per DAO.

### 3.2.2 Open issues

1. How to securely store the private key for the DAO contract?
    1. Maybe I could generate the private key every time? But how could I define people who already voted if their private key will always be different (as well as the public key, obviously)?
    2. The smart contracts (or only wallets) already have private and public keys. That may be enough. Or will I not be able to use these values?

## 3.3 Idea 2

### 3.3.1 Short description

There are two main parties: a User and a DAO. A DAO creates a poll, Users vote there, and the final poll result is made public, but the voters themselves are untraceable.

### 3.3.2 Full description

1. The user creates a response / an answer, which is then encrypted with Pedersen Commitments (to make it infeasible to brute-force the answer the user provided).

<aside> 💡 The answer can also be verified with the ZKP after the commitment is created to make sure that this answer is legitimate (but is there even a point in this?)

</aside>

2. The answer (the commitment) is sent to the DAO party, which checks the answer and adds it to the poll. But the DAO should not be able to know who sent the message.

<aside> 💡 As an idea, I can have a list of people that are a part of a DAO (a list of wallet addresses) and then, with ZKP, verify that the person who sent an answer to the poll is a part of the DAO, but who he is will not be revealed.

</aside>

3. After submitting the answer, the user will get a (Shnorr) signature (somehow, it could be in a manifestation of NFT or POAP). Shnorr signature solves two issues at a time:
    1. The user will not be able to vote multiple times.
    2. Everyone can verify that the person has voted for a specific poll.
4. The poll should be hidden so that no one can track who voted. If possible, it will be done on-chain. Otherwise, the poll will stay off-chain until the end of the poll.

### 3.3.3 Further ideas

1. Instead of sending a number of the option in the questionary/poll, send a Pedersen Commitment. (Or hashed Pedersen Commitment, as a simple Pedersen Commitment could be easily checked (or not?) with arkworks). It will be sent to the Smart Contract and stored there (so that it is verifiable), and then this answer will be sent to the backend to learn what the person voted for and then send this vote to the DAO Smart Contract.

## 3.4 Idea 3

### 3.4.1 Description

This idea involves the usage of Merkle Trees. A Poll will be another entity, additionally to User and DAO. A Poll will be a Merkle Tree of all Users in the DAO. And every leaf of the tree will contain not only a user but also their answer. It would work the following way:

1. A User submits an answer (could be encrypted with Pedersen Commitments, if needed for security reasons).
2. The answer is submitted to the Poll and verified by ZKP. As inputs, there will be an initial root of the Merkle tree (Poll) and the final root of the tree. The answer of the user would be the witness there.
3. If all the checks are passed, the new root of the Poll will be added to the DAO smart contract, and the old root of the Poll will be deleted. (Potentially, the root will be submitted only in the end to make it more secure (as otherwise others will be able to see when somebody voted, although they still will not know the user’s answer, so maybe it’s not that relevant)).
4. The Polls will be stored off-chain. The poll results will be shown only at the end of the poll and will not be visible while voting.
5. Moreover, thanks to the Merkle Trees, it may be possible to verify that a User is a part of a DAO through membership verification. So, the Merkle Tree usage would kill two birds with one stone.