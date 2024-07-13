# Whitelist-Gated Token Sale Program

This Solana program allows users to participate in a whitelist-gated sale for a new token. The token price is static, and there is a purchase limit per wallet address.

## Overview

The program provides three primary instructions:
1. **Initialize Sale**: Set up the sale with a specific token price and maximum tokens per wallet.
2. **Add to Whitelist**: Add users to the whitelist to allow them to participate in the sale.
3. **Buy Tokens**: Allow whitelisted users to purchase tokens, adhering to the maximum limit per wallet.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor](https://project-serum.github.io/anchor/getting-started/installation.html)

## Installation

1. **Clone the repository**:
    ```sh
    git clone https://github.com/your-repo/whitelist-token-sale
    cd whitelist-token-sale
    ```

2. **Build the Program**:
    ```sh
    anchor build
    ```

3. **Deploy the Program**:
    ```sh
    anchor deploy
    ```

## Instructions

### Initialize Sale

Initialize the token sale with a specific price and maximum tokens per wallet.

**Parameters**:
- `price`: The price of each token in lamports.
- `max_per_wallet`: The maximum number of tokens each wallet can purchase.

**Example**:
```typescript
await program.rpc.initializeSale(new anchor.BN(price), new anchor.BN(maxPerWallet), {
  accounts: {
    sale: salePDA,
    tokenMint: tokenMint,
    seller: seller.publicKey,
    sellerTokenAccount: sellerTokenAccount,
    systemProgram: anchor.web3.SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
    rent: anchor.web3.SYSVAR_RENT_PUBKEY,
  },
  signers: [seller],
});
