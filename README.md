# Astro Gate

**Astro Gate** is a minimalist *open bank* built on Solana.
Users can only deposit and withdraw Solana tokens as of current version. Other tokens may be implemented in future versions.

## Version
### 0.3.0

Astro Bank is built with simplicity in mind. Currently it works with only two separate entrypoints.
Its main focus in efficiency and layering, which means that while this is still an alpha version, it is already fully operable.

## Mint

Mint funds to be allocated will be automatically transferred to the vault by the Candy Machine. There may be several deployments before the mint, and the program address is hence subject to change. Only send withdrawal requests through the official website to avoid any issues.

## Accounts

A program derived address is created for each client. That's where tokens are stored.
Tokens can be deposited by anyone with a regular system transfer to the relative program address. All available balance is available as soon as the transaction is confirmed.
Tokens can be transferred to and from any account by any wallet.
Withdrawals can only be made to the public key of the account owner.

## Lock period

The lock period is currently of 256 days — calculated from the account creation timestamp.
It's worth noting that the account is automatically eliminated and created anew everytime the entirety of funds (*rent exemption*) is removed.

## Authority

Authority is verified against ownership of the *owner* NFT.
The owner can only *drain* funds from an account after the lock period is over, which makes it so that users have enough time to claim the rewards allocated from the mint.
