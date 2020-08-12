---
id: consensus
title: Consensus
---

In order for a new block to be included on Aleo, consensus on the canonical block must be reached by the network.
To reach this agreement on Aleo, a consensus mechanism called Proof of Succinct Work is used to achieve this.

## Proof of Succinct Work

Proof of Succinct Work is a SNARK-based Proof of Work algorithm, designed to incentivize the development of
hardware-accelerated SNARKs.

## Mining on Aleo

Mining is a permissionless process of producing new [blocks](04_blocks.md) for inclusion on Aleo.
Miners produce blocks by processing pending [transactions](03_transactions.md) and computing a valid
nonce for solving a [Proof of Succinct Work](05_consensus.md#proof-of-succinct-work) puzzle.

On success in finding a valid block, miners are compensated with a [block reward](#block-rewards) for their contribution. To
ensure Aleo continues to advance, a difficulty rate is dynamically adjusted to reflect the number
proofs per second that miners on Aleo contribute.

### Block Rewards

A block reward is the total amount of Aleo credits rewarded to the address that mined a block.
This value is the base block reward in addition to the fees paid by all transactions included in the block.

|      Block Number     |   Reward  |
|:---------------------:|:---------:|
| 0 - 3,503,999         | 150 ALEO  |
| 3,504,000 - 7,007,999 | 75 ALEO   |
| 7,008,000 - ∞         | 37.5 ALEO |

Initially, each Aleo block reward is worth 150 Aleo credits. This block reward is halved after every 3,504,000 blocks, which
is approximately four years at an estimated 100 blocks per hour. After two iterations of halving the block reward, it will
remain at 37.5 for perpetuity. 

### Block Difficulty and Block Times

The block time is the amount of time it takes for the network to produce a valid block.
This block time is variable and based on the network's hashrate, but regulated by the block difficulty. 
The block difficulty is adjusted according to the most recent block times in order to regulate and
stabilize the average block time of the network.
