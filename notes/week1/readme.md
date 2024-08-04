# Class 1

---

We have covered basics of the solana environment in that class and at the end of the class, `spl-token` is used to demonstrate creating a fungible token.

- [Account](#account)
- [Programs](#programs)
- [PDA (Program Derived Account)](#pda)
- [Transaction](#transaction)
- [SPL Token](#spl-token)

### [Account](#account)

---

All data is stored in `accounts` on Solana. You can think of it like a **key-value** pair on the database and each entry is called as **account**

![accounts](/notes/week1/images/accounts.svg)

_Key-value pair demonstration from [Solana](https://solana.com/docs/core/accounts)_

| Account Type    | Definition                                                        |
| --------------- | ----------------------------------------------------------------- |
| Data Account    | It stores the state of the program                                |
| Program Account | It just stores the executable program code, not the state itself. |

> [!IMPORTANT]
> Accounts can store up to **10MB** of data, which can consist of either _executable program code_ or _program state_.

> [!WARNING]
> Accounts require a [**rent**](https://solana.com/docs/core/fees) deposit in SOL, proportional to the amount of data stored, which is fully refundable when the account is closed.
> **Rent-exemption** is required on Account creation.

There is an `ownership terminology` in Solana which an account can only be created by the **System Program**. That is, owner of the account is actually System Program but it gives ownership to the related address that call that system program.

How an account info object looks like👇

```js
{
key: number,
lamports: number,//Account's balance in lamports
data: Uint8Array, //If the account is a program account, then data stores only executable program code
is_executable: boolean, // Whether account is executable program or not
owner: PublicKey,
}
```

### How can we reach those accounts? 🤔

Each account is identifiable by its unique address, represented as 32 bytes in the format of an [**Ed25519**](https://cryptography.io/en/latest/hazmat/primitives/asymmetric/ed25519/) `PublicKey`. You can think of the address as the unique identifier for the account.

![accounts](/notes/week1/images/account-address.svg)

_Account address demonstration from [Solana](https://solana.com/docs/core/accounts) Docs_

### [Programs](#programs)

---

Program accounts are marked as executable and they store only executable logic(code), not state itself. They refers to `smart contracts`

➡️They can interact with non-executable accounts when they need to read or write the state(data).

> [!NOTE]
> Program owner is the address which loads the program to the on-chain.

There are two types of programs
|Program|Definition|Example|
|-----|-----|----|
|Native Programs|They are provided by Solana itself|`System Program`,`BPF Loader Program`, |
|User Programs|They are written by us|many examples there are|

### How to Write a Program?

➡️ Generally, `Rust 🦀` programming language is used to write smart contracts on the Solana ecosystem.
You can use one of the below tools to write a program,

- Native Rust
- Anchor (Beginner friendly)

### How to Interact with a Program? - IDL

If there is a written program, it generates an **IDL (Interface Design Language)** during build process. By using that IDL, on-chain programs can be reachable on the client side.

👉You can examine an IDL example on the [ts-program](/ts/programs/wba_prereq.ts) file in that repo

> [!NOTE]
> They are just like an ABI on the Ethereum environment where solidity generally using. `JSON` formatting is used on IDLs and developers can easily understand the requirements of the program

### [PDA (Program Derived Account)](#pda)

They are addresses that're deterministically derived and look like standard public keys, but have no associated private keys.

> [!IMPORTANT]
> Since they don't have a **Keypair** like wallet, they're bumped off the _Ed25519 curve_

👉 A PDA can then be used as the address for an on-chain account, providing a method to easily store, map and fetch program state.
==PDA enables us to easily find the address of an account at a later time.==

![Off Curve PDA](/notes/week1/images/address-off-curve.svg)

_Off Curve PDA demonstration from [Solana](https://solana.com/docs/core/pda) Docs_

To derive a PDA,

- We use `findProgramAddress(seed,programId)` function from `@solana/web3.js` ,

> [!IMPORTANT]
> If an account is using a PDA as its address, it must be **explicitly** created through a dedicated instruction within a solana program.
