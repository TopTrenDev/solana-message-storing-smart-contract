# Solana Message Storing Smart Contract (Rust, Anchor framework)

A minimal smart contract for storing, updating, and reading a string message on Solana (Anchor Framework)

[![Telegram](https://img.shields.io/badge/Telegram-@TopTrenDev_66-black?style=for-the-badge&logo=telegram&logoColor=2CA5E0)](https://t.me/TopTrenDev_66)
[![Twitter](https://img.shields.io/badge/Twitter-@toptrendev-black?style=for-the-badge&logo=twitter&logoColor=1DA1F2)](https://x.com/toptrendev)
[![Discord](https://img.shields.io/badge/Discord-toptrendev-black?style=for-the-badge&logo=discord&logoColor=5865F2)](https://discord.com/users/648385188774019072)

## Overview

This project demonstrates a simple Solana program that:

- Initializes an on-chain account with a default message ("Hello, Solana!")

- Allows updating the stored message

- Allows client-side reading of the message

- Uses Rust + Anchor framework

- Includes proper account creation, serialization, and storage handling

## Rust, Anchor Smart contract Environment

- anchor version

```
anchor-cli 0.32.1
```

- solana version

```
solana-cli 3.0.10 (src:96c3a851; feat:3604001754, client:Agave)
```

- cargo version

```
cargo 1.93.0-nightly (25d319a0f 2025-11-11)
```

## Deployment Guide

- Build the Program

```
anchor build
```

- Deploy to Devnet(Mainnet)

```
anchor deploy
```

## Client Usage (TypeScript)

### Install client dependencies

```
npm install
```

### Client Test

```
npm run test
```

Or

```
anchor test --skip-build --skip-deploy
```

## Wallet Transaction References (Devnet)

Track the key on-chain messages for this wallet:

[![Solscan](https://img.shields.io/badge/View_on_Solscan-Initialize_Message-14F195?style=for-the-badge&logo=solana&logoColor=white)](https://solscan.io/tx/37qbT1i9EobjHakPhYT7PseBm99pqJeJrMc3araZFjasg1RQUrAEdrEPMXRfHKLMGdds7xUkA9jfjjszZf1aEAmu?cluster=devnet)

[![Solscan](https://img.shields.io/badge/View_on_Solscan-Update_Message-14F195?style=for-the-badge&logo=solana&logoColor=white)](https://solscan.io/tx/4EoT5ywK8qGSJZfZF9yFJVuCZ467dT87LfikRL4a62B296uNh27WxgKrCWQVbPBnQtmDFZMWBYtxY6N3Jrwxjenc?cluster=devnet)

