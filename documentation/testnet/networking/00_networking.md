---
id: networking
title: Networking
sidebar_label: Networking
---

The snarkOS network protocol establishes a peer-to-peer network of nodes that maintain the liveness of the ledger by actively exchanging transactions and blocks.

## Introduction

snarkOS creates a simple broadcast network that uses TCP to handle all data transfers. snarkOS full nodes download and verify all blocks and transactions before becoming an active node in the network. 

## Node/Peer Discovery

When a node joins the network for the first time, it doesn't know any of the IPs of the currently active nodes in the network. 
In order to bootstrap node discovery, there are hardcoded bootnodes in the snarkOS implementation. 

Once a node is connected to the network through one or more nodes, it can further scan the network in search of new peers. 
This processes starts with the nodes asking their peers for other connected nodes in the network with a `GetPeers` message followed by establishing connections with the newly discovered peers.
This discovery process is fully decentralized and does not require any central coordination of connection handling.

snarkOS also stores the IP of connected peers in its database; this allows the node to connect directly to peers without having to use bootnodes on subsequent startups.

Additionally, node operators can connect to peers manually by using the command line to connect to known IPs.

#### Bootnodes

Bootnodes operate like any other full node, however their IP's are hardcoded into the snarkOS implementation, which effectively makes them public access points for any and all nodes in the network.
Bootnodes are run by community members and bolster the network by allow new nodes to join the network effortlessly.

## Connecting to Peers

Peer connections are established with a handshake. A valid handshake begins with a `Version` message that includes the node's version, block height and current timestamp. The receiver returns with its own `Version` message. After which, both nodes send a `Verack` message acknowledging the recieved `Version` message and establishes a peer connection.

Peer connections are maintained with a Ping-Pong Protocol that relays `Ping` and `Pong` messages to ensure that peers are still connected.

## Block Download/Sync

Before nodes can actively participate in the network, it must have the latest state of the ledger. Whether a node is newly connecting to the network or just has stale state, it must download it's missing blocks by syncing with another node. 

snarkOS uses a "Header-First" approach to block syncing, where the node downloads and validates the block headers before downloading the full blocks in parallel. 
 
When a node determines it needs to download state, it selects a peer as the sync-node and sends it a `GetSync` message. The get sync message contains information about the node's current block state so the sync node can determine which block headers need to be sent as a response.  

Upon receiving a `GetSync` message, the sync-node sends back at most 100 block headers via a `Sync` message. The requester then validates these headers and downloads the blocks in parallel by sending out `GetBlock` messages. After these blocks have been downloaded, the requestor sends another `GetSync` message and repeats the cycle until it's chain is up to date.

Here is a basic iteration of the sync protocol:

|   Message  |   Sender  |  Receiver | Data                                |
|:---------- |:---------:|:---------:|-------------------------------------|
| `GetSync`  | Node      | Sync Node | 1 or more block hashes              |
| `Sync`     | Sync Node | Node      | Up to 100 new block headers         |
| `GetBlock` | Node      | Any Peer  | Block header of the requested block |
| `Block`    | Any Peer  | Node      | A serialized block                  |

## Memory Pool

Full nodes need to keep track of transactions that are eligible to be included in future blocks. Because these unconfirmed transactions are not yet included in the ledger, the node stores them in memory, hence the memory pool.

Transactions are removed from the memory pool when the node is shut down or when the transactions are included in valid blocks. 

Transactions that are included in stale blocks, can be re-added into the memory pool because they no longer conflict with a transaction on the longest chain. 

## Transaction Broadcasting

A node can broadcast a transaction by sending a `Transaction` message to its connected peers. The peers receiving the transaction validate the transaction and can further propagate the transaction by broadcasting it to their connected peers. Eventually, the transaction should reach every connected peer in the network.

## Block Broadcasting

A node can broadcast a block in the same way it broadcasts a transaction by using a `Block` message.

## Message Types

A list of message types can be found [here](01_message_types.md).
