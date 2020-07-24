---
id: account
title: Accounts
sidebar_label: Accounts
---

An **Aleo account** is comprised of an [account private key](#account-private-key), [account view key](#account-view-key),
and an [account address](#account-address).

The account private key is used to authorize a transaction, which updates the global state of account records. The account
view key is used to decrypt account records, which are encrypted under the user's account address. Lastly, the account
address enables users to interact with one another, sending and receiving records that encode values and application data.

To protect user *assets* and *record data*, one should **never disclose their account private key** to any
third parties. For real-world applications on Aleo, users should derive an [account delegation key](#delegation-key)
from their account private key to allow third parties to *trustlessly* run applications and generate transactions
on a user's behalf.

## Account Private Keys

Every Aleo account private key is constructed from a randomly-sampled **account seed**. This account seed is used to generate:
- a **secret key** for the account signature scheme,
- a **pseudorandom function seed** for transaction serial numbers, and
- a **commitment randomness** for the account commitment scheme.

### Private Key Format

```
AKEY1b47dMA8f9GfXPsW9s16qWfiYYmWGAAcorK9RkaVpBeFA
```

An account private key is formatted as a Base58 string, comprised of 49 characters.
The account private key is encoded with a [private key prefix](#account-prefixes) that reads `AKEY1`, indicating
that it is a private key and should not be shared with other users.

### Delegation Key

To facilitate user interactions at scale with ease, users can derive an account delegation key which allows a third-party to
trustlessly process applications and user transactions on the user's behalf.

The account delegation key is comprised of:
- a **public key** for the account signature scheme,
- the **pseudorandom function seed** from the account private key, and
- the **commitment randomness** from the account private key.

While the account delegation key does **not** allow a third party to spend assets or forge record data, it does allow the
party to access and view it. As such, users should provide this key only to authorized parties.

## Account View Key

An Aleo account view key is derived from an account private key and enables users to decrypt their
[records](02_transactions.md#record-ciphertexts) from the global ledger.
As account view keys are able to access every record in a user's account, this key can be used by
third-party auditors to verify the complete history of an account.

The account view key is comprised of:
- a **secret key** for the account encryption scheme.
 
## Account Address

An Aleo account address is a unique identifier that allows users to transfer value and record data to one another in transactions.

The account address is comprised of:
- a **public key** for the account encryption scheme.

### Address Format

```
aleo1y90yg3yzs4g7q25f9nn8khuu00m8ysynxmcw8aca2d0phdx8dgpq4vw348
```

An account address is formatted as a Bech32 string, comprised of 63 characters.
The account address is encoded with an [address prefix](#account-prefixes) that reads `aleo1`.

## Advanced Topics

### Offline Accounts

In many instances such as enterprise settings, it is advisable to handle sensitive keys and data on isolated, offline machines.
An Aleo account can be created on an offline machine and available for immediate use. In conjunction with account delegation keys,
a user can ensure their private key remains offline even for creating transactions.

While no solution is perfect, it is advisable to create a new Aleo account on a disconnected device to minimize the risk of
leaking one's account private key to unintended parties.

### Account Commitment Outputs

An account view key is comprised of an encryption secret key, which is derived from the account commitment output.
As the encryption secret key is a scalar field element, and the account commitment output is a group element, an efficiently 

The account commitment output is used to create an account view key. 
This encryption secret key is a scalar field element derived from the account commitment output. To ensure the validity
of the account view key, the account commitment output should be representable in the scalar field

### Create an Account

Given global instantiated Aleo [parameters and algorithms](06_parameters.md). 

#### Generate a Private Key 

1. Sample a 32 byte `seed` from random
    
2. Construct private key components
    - `sk_sig` = Blake2s(`seed`, 0)
    - `sk_prf` = Blake2s(`seed`, 1)
    - `r_pk` = Blake2s(`seed`, 2)
    
3.`private_key` = (`seed`, `sk_sig`, `sk_prf`, `r_pk`)

#### Generate a View Key 

1. Construct `pk_sig` = AccountSignature,GeneratePublicKey(<code>pp<sub>account_sig</sub></code>, `sk_sig`)

2. `view_key` = AccountCommitment.Commit(<code>pp<sub>account_cm</sub></code>, (`pk_sig`, `sk_prf`), `r_pk`)

#### Generate an Address

1. `address` = AccountEncryption.GeneratePublicKey(<code>pp<sub>account_enc</sub></code>, `view_key`)

### Account Diagram

```mermaid
graph TD
	A["Seed (32 Bytes)"] 
    A --> |"Blake2s(Seed, 0)"| B(sk_sig)
    A --> |"Blake2s(Seed, 1)"| C(sk_prf)
    A --> |"Blake2s(Seed, 2)"| D(r_pk)
    
    B --> E(Account Private Key)
    C --> E(Account Private Key)
    D --> E(Account Private Key)
    
    E --> F(Account View Key) 
    F --> G(Account Address) 
```

## Account Prefixes

|             |  Type  |       Prefix      |
|:-----------:|:------:|:-----------------:|
| Private Key |  bytes | [21, 38, 63, 229] |
|   Address   | string |       "aleo1"     |
