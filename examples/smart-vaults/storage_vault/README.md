# Storage Vault

A Smart Vault that stores who deposited and how much and lets only them withdraw exactly that much again

## Prerequisites

- Rust toolchain with `wasm32v1-none` target
- Node.js 18+

## Step-by-step: Use on WASM Devnet

This guide uses the public Devnet WASM endpoint at `wss://wasm.devnet.rippletest.net:51233`.

### 1. Install dependencies

```shell
npm install
```

### 2. Build the WASM

```shell
cargo build --target wasm32v1-none --release
```

Artifact:

```
./target/wasm32v1-none/release/storage_vault.wasm
```

### 3. Deploy and test on Devnet

Use the test script to deploy a vault and test the code.

```shell
cd ../../..
./scripts/run-tests.sh examples/smart-vaults/storage_vault
```

This will:

- Connect to WASM Devnet
- Create and fund two wallets (Origin and Destination)
- Create an VaultCreate transaction with your compiled code
- Attempt to deposit into the vault
- Attempt to withdraw from the vault

Expected result: `tesSUCCESS`

