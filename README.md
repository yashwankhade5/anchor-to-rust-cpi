# Anchor to Rust CPI Example

This project demonstrates how to perform **Cross-Program Invocation (CPI)** from an **Anchor program** into a **native Rust Solana program**. It also includes tests for both the native Rust contract and the Anchor contract.  

---

## Project Structure

The project root contains three main folders:

1. **`rust_contract`** – Contains the native Rust Solana program.
2. **`anchor_contract`** – Contains the Anchor program that performs CPI to the Rust contract.
3. **`client`** – Contains tests for the native Rust program.

---

## Overview

- The **Rust contract** contains the actual logic to be executed on Solana.
- The **Anchor contract** acts as a client program that performs **CPI** into the Rust contract to execute transactions or operations.
- **Anchor itself does not perform any operations** by default; it simply facilitates CPI calls.
- Tests for the **native Rust program** are written in the `client` folder.
- Tests for the **Anchor program** are written inside the `anchor_contract/tests` folder.

### Note on Local Validator

- By default, `anchor test` spins up its own Solana validator.
- In this project, the validator was **manually started** because the project was being developed on **WSL** and the validator did not start properly when the project folder was mounted in Windows.
- Instead of using Anchor’s default validator, a **local validator** was started manually and Anchor was pointed to it.

---

## Setup & Running the Project

### 1. Build and Test the Rust Contract

1. Navigate to the `rust_contract` folder:  

 ```bash
   cd rust_contract
```
2. Build the program:
```bash
cargo build-sbf
```
3. Navigate to the client folder:
```bash
cd ../client
```

4. Install dependencies:

```bash
bun install
```

5. Run tests for the Rust contract:

```bash
bun test
```
### 2. Build and Test the Anchor Contract

1. Navigate to the anchor_contract folder:

```bash
cd ../anchor_contract
```

2. Build the Anchor program:
```bash
anchor build
```

3. Start a local Solana validator:
```bash
solana-test-validator
```

4. Deploy the Rust contract to the blockchain: 
(replace `<filename>`.so with the actual file name generated in rust_contract/target/deploy/)
```bash
solana program deploy ../rust_contract/target/deploy/<filename>.so
```

   - Copy the program ID given by the deploy command.
 
   - Paste it into the tests inside the anchor_contract tests folder.

5. Run tests for the Anchor contract:
```bash
anchor test

```

## Key Points

 - Anchor facilitates CPI calls to native Rust programs.

 - PDAs and accounts are automatically set to the program ID in declare_id!.

 - Using a custom local validator is necessary on WSL when default Anchor validators fail to start.

## Notes

 - Ensure the Rust program is deployed before running the Anchor tests.

 - Always confirm the program ID of the deployed Rust contract and update the tests accordingly.

 - The project is designed to help understand Anchor-to-Rust CPI and test both programs independently.
