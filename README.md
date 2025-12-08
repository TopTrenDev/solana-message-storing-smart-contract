# Solana Message Storing Smart Contract (Rust, Anchor framework)

A minimal smart contract for storing, updating, and reading a string message on Solana (Anchor Framework)

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
