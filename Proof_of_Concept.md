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
<!-- TOC -->

# Introduction

Firstly, the Proof of Concept is a realization of a certain method or idea in order to demonstrate its feasibility. ([source](https://en.wikipedia.org/wiki/Proof_of_concept))

This document shows the details of the Proof of Concept: the idea, how it works, and how it is implemented. Moreover, the previous Proof of Concept ideas are also mentioned here in case anyone finds them attractive and/or decides to develop them.

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
6. The $hash$ value is calculated using the following equation: $wallet * pass = hash + word$, so $hash = wallet * pass - word$.
7. The calculated $hash$ value is then submitted to the smart contract, which has a mapping that keeps track of the user $wallets$ and user calculated $hashes$.

### 1.1.3 Zero-Knowledge verification

1. *(Prerequisite)* The user has to connect their wallet to the website. Otherwise, the verification will fail.
2. Firstly, the $hash$ is retrieved from the smart contract, according to the user $wallet$.
3. Then the user $password$ is hashed.
4. Then this hash is separated into two parts: $pass$ and $word$.
5. The $pass$ and $word$ are also hashed.
6. In the end, we submit these values, use the ***arkworks***  for their verification, and get a *bool* as a response: *true* if the password is correct, *false* if it is not.
    1. If you want to learn how the password is verified using ***arkworks*** (which utilizes *zk-SNARK* logic), check [Arkworks](#21-arkworks).

## 1.2 Potential advantages

### 1.2.1 Rainbow table resistance

If you don't know what a Rainbow table is, check the [source](https://en.wikipedia.org/wiki/Rainbow_table) first. 

The public hash stored on a smart contract is never the same for any pair of users. This is because the public hash calculation depends on the user’s wallet address, which is unique. If two users have the same password, their public hashes will differ.

### 1.2.2 Random final output

***Prerequisite:*** Check the [Arkworks](#21-arkworks) before reading further for better understanding.

When verifying a password with ***arkworks*** in the end, two Pedersen commitments are compared with each other $commitment_a == commitment_b$, where $commitment_a = C(hash)$ and $commitment_b = C(wallet * pass - word)$. Here $C()$ is a function that converts an input into a corresponding Pedersen commitment. Let’s say the final commitments should equal a particular final value $final = commitment_a = commitment_b$. It would be easier to hack the password if the hacker knew this $final$ value. However, when calculating commitments, a random value is always used, which means that during every new verification, the same input: $hash$, $wallet$, and $password$ will produce different commitments $commitment_{a_1}, commitment_{a_2}, commitment_{b_1}, commitment_{b_2}$, where $commitment_{a_1} ~!= commitment_{a_2}$ and $commitment_{b_1} ~!= commitment_{b_2}$, while $commitment_{a_1} == commitment_{b_1}$ and $commitment_{a_2} == commitment_{b_2}$. This also means that the corresponding final values $final_1$ and $final_2$ will never be equal.

### 1.2.3 API features

The backend part of the application, which is responsible for Public hash generation and password verification with Zero-Knowledge Proof (ZKP), acts as an API. The smart contract that stores public hashes in it is shown only as an example. It can be substituted with any other smart contract, which has the mapping of $wallet$ - $hash$ pairs and methods for adding new $hashes$ and getting already existing ones. Thus, other web3 projects and organizations can use Zero-Knowledge Password Manager functionality in their own smart contracts. 

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

The combination of this password manager and the Shnorr signature can also be seen as a web3 version of the two-step verification.

_**Note:**_ This idea may be redundant, as the smart contract side maps the public hashes with wallet addresses, and the hash is returned according to the `msg.sender` parameter. This means that every time a person verifies their password, the smart contract sends the public hash according to the user’s wallet address. So, the smart contract does the user verification for us.    

# 2 Details

## 2.1 Arkworks

This is how the password verification happens in the ***arkworks*** section after receiving the values $hash$, $pass$, $word$, and $wallet$.

It will be separated into two smaller parts, [Circuit](#211-circuit) and [Groth16](#212-groth16), to make understanding the workflow of Zero-Knowledge usage easier.

1. However, a Pedersen commitment ( $commitment_a$ ) is created with a $hash$ as an input. This happens before any *Circuit* or *Groth16* logic.

### 2.1.1 Circuit

*The Circuit* section explains, what happens in the circuit, that is sent to the Zero-Knowledge set up to verify the password. In this context, you can see a *“circuit”* as a function that defines the verification code that should be executed and that defines *private variables (witnesses)*, *constants*, and *public inputs* for the Zero-Knowledge Proof setup.

1. All three values: $pass$, $word$, and $wallet$ are declared as *witnesses.*
2. Then we calculate the $hash_2$ value from the following equation: $wallet * pass = hash + word$, which means that the $hash_2$ value is equal to $hash_2 = wallet * pass - word$.
   1. The values $hash_2& and $hash$ are **_not_** the same! The $hash$ was calculated in the first password insertion. The $hash_2$ value is created during the verification process to prove that the password is correct. The $hash_2$ is calculated with the same equation as $hash$. So, **_only if_** the password is correct the following will be true: $hash_2 = wallet * pass - word = hash$ if the password is wrong $hash_2 != hash$. 
3. After that, the $hash_2$ value is used to create another Pedersen commitment: $commitmnet_b$.
4. In the end, the circuit checks the validity of the following expression: $commitment_a == commitment_b$.

### 2.1.2 Groth16

This section shows how the Zero-Knowledge setup is constructed and how it works in this application.

1. First of all, a circuit with dummy data is created. (reason for dummy data is explained later)
2. Then this circuit is supplied to the Groth16 setup function, which returns a proving key ( $pk$ ) and a verification key ( $vk$ ).
    1. It is needed to supply a circuit for the setup because $pk$ and $vk$ are also connected to a particular circuit structure. It means that if you have a different circuit and $pk_2$ and $vk_2$ generated by supplying this circuit, when you will try to use a pair $pk$ and $vk_2$ or a pair $pk_2$ and $vk$, you will get an Error and the verification will fail.
    2. Also, we provide dummy data here because the provided data is irrelevant in this step. The $pk$ and $vk$ are only bonded with the structure of the circuit, and the data is not taken into consideration during the bonding process. The only requirement from the data is to be correct (meaning that it should pass the checks in the circuit); otherwise, the Error will be thrown from the circuit itself.
3. After key generation, the circuit with actual data is created.
4. This circuit and the proving key, $pk$, are used to generate proof.
    1. Note that in this step, the circuit logic with all the provided data is checked before the proof generation. If it fails, an error is thrown, and the program doesn’t move further.
5. The generated proof, verification key $vk$, and the public inputs are provided to the verification method. The method returns *a bool*. If it is true, then the user successfully passed the ZK check. If it’s false, then it is considered that the user doesn’t know the password.
    1. As public inputs, the coordinates of the Pedersen commitment $commitment_a$ are provided, as a Pedersen commitment can also be interpreted as a point on an elliptic curve.

