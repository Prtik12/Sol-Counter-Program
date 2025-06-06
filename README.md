# Solana Counter Program

This is a simple smart contract written in Rust for the Solana blockchain.

## What it does

The contract maintains a counter on-chain. It supports two instructions:

- `Increment(value)`: Increases the counter by a specified value.
- `Decrement(value)`: Decreases the counter by a specified value.

The counter state is stored using Borsh serialization.

## Why this project

This was created as a first Solana contract to understand:

- How Solana programs work
- How instruction data is handled
- How account data is read and written
- How to use Borsh for serialization

## Technologies Used

- Solana Program Library
- Rust
- Borsh (for serialization)

## Notes

It was built purely for learning and experimenting with Solana development.
