# Anonymous Voting Ideas

# Table of Contents

<!-- TOC -->
* [Introduction](#introduction)
* [1 Idea 1](#1-idea-1)
   * [1.1 Description](#11-description)
   * [1.2 Open issues](#12-open-issues)
* [2 Idea 2](#2-idea-2)
  * [2.1 Short description](#21-short-description)
  * [2.2 Full description](#22-full-description)
  * [2.3 Further ideas](#23-further-ideas)
* [3 Idea 3](#3-idea-3)
   * [3.1 Description](#31-description)
<!-- TOC -->

# Introduction

The Anonymous Voting project was initially supposed to be this Proof of Concept. However, due to a lack of time, it was decided to switch to the Zero-Knowledge Password Manager. The Anonymous Voting section will present the main ideas behind this project for those curious or who choose to develop this idea.

Further, there will be nothing about Zero-Knowledge Password Manager. This section is dedicated specifically to Anonymous Voting.

# 1 Idea 1

## 1.1 Description

It is going to be a whole application/website for that. Every account will have private and public keys (1 public key and 1 private key per account). *It is worth mentioning that DAOs are also accounts in this case.* Then every time a person votes on the app, their value is encrypted with Pedersen commitment and Twisted ElGamal Encryption and sent to the DAO contract. That way, everyone can see that the person voted, but nobody will know the exact value or which option the person was voting for.

We may also have Merkle Trees that will store accounts for every DAO, which means there will also be a Merkle Tree per DAO.

## 1.2 Open issues

1. How to securely store the private key for the DAO contract?
    1. Maybe I could generate the private key every time? But how could I define people who already voted if their private key will always be different (as well as the public key, obviously)?
    2. The smart contracts (or only wallets) already have private and public keys. That may be enough. Or will I not be able to use these values?

# 2 Idea 2

## 2.1 Short description

There are two main parties: a User and a DAO. A DAO creates a poll, Users vote there, and the final poll result is made public, but the voters themselves are untraceable.

## 2.2 Full description

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

## 2.3 Further ideas

1. Instead of sending a number of the option in the questionary/poll, send a Pedersen Commitment. (Or hashed Pedersen Commitment, as a simple Pedersen Commitment could be easily checked (or not?) with arkworks). It will be sent to the Smart Contract and stored there (so that it is verifiable), and then this answer will be sent to the backend to learn what the person voted for and then send this vote to the DAO Smart Contract.

# 3 Idea 3

## 3.1 Description

This idea involves the usage of Merkle Trees. A Poll will be another entity, additionally to User and DAO. A Poll will be a Merkle Tree of all Users in the DAO. And every leaf of the tree will contain not only a user but also their answer. It would work the following way:

1. A User submits an answer (could be encrypted with Pedersen Commitments, if needed for security reasons).
2. The answer is submitted to the Poll and verified by ZKP. As inputs, there will be an initial root of the Merkle tree (Poll) and the final root of the tree. The answer of the user would be the witness there.
3. If all the checks are passed, the new root of the Poll will be added to the DAO smart contract, and the old root of the Poll will be deleted. (Potentially, the root will be submitted only in the end to make it more secure (as otherwise others will be able to see when somebody voted, although they still will not know the user’s answer, so maybe it’s not that relevant)).
4. The Polls will be stored off-chain. The poll results will be shown only at the end of the poll and will not be visible while voting.
5. Moreover, thanks to the Merkle Trees, it may be possible to verify that a User is a part of a DAO through membership verification. So, the Merkle Tree usage would kill two birds with one stone.