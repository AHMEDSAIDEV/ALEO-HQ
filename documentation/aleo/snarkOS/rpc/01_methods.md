---
title: snarkOS JSON-RPC Methods
sidebar_label: RPC Methods
---

## Public Methods

These RPC methods are public and do not require authentication.

### getblock

Returns information about a block from a block hash.

#### Arguments

|  Parameter   |  Type  | Required |              Description              |
|:------------ |:------:|:--------:|:------------------------------------- |
| `block_hash` | string |    Yes   | The block hash of the requested block |

#### Response

|      Parameter      |  Type  |                    Description                    |
|:------------------- |:------:|:------------------------------------------------- |
| `block_hash`        | string | The number of blocks in the best valid chain      |
| `difficulty_target` | number | The difficulty of the block                       |
| `hash`              | string | The block hash (same as provided)                 |
| `height`            | number | The block height                                  |
| `merkle_root`       | number | The merkle root of the transactions in the block  |
| `nonce`             | number | The nonce for solving the PoSW puzzle             |
| `proof`             | string | The Proof of Succinct Work                        |
| `size`              | number | The size of the block in bytes                    |
| `time`              | number | The block time                                    |
| `transactions`      | array  | The list of transaction ids included in the block |

#### Example
```
curl --data-binary '{"jsonrpc": "2.0", "id":"documentation", "method": "getblock", "params": ["caf49293d36f0215cfb3296dbc871a0ef5e5dcfc61f91cd0c9ac2c730f84d853"] }' -H 'content-type: application/json' http://127.0.0.1:3030/
```

### getblockcount

Returns the number of blocks in the best valid chain.

#### Arguments

None

#### Response

| Parameter |  Type  |                  Description                 |
|:---------:|:------:|:--------------------------------------------:|
| `result`  | string | The number of blocks in the best valid chain |

#### Example
```
curl --data-binary '{"jsonrpc": "2.0", "id":"documentation", "method": "getblockcount", "params": [] }' -H 'content-type: application/json' http://127.0.0.1:3030/
```

### getbestblockhash

Returns the block hash of the head of the best valid chain.

#### Arguments

None

#### Response

| Parameter |  Type  |                  Description                  |
|:---------:|:------:|:---------------------------------------------:|
| `result`  | string | The block hash of the most recent valid block |

#### Example
```
curl --data-binary '{"jsonrpc": "2.0", "id":"documentation", "method": "getbestblockhash", "params": [] }' -H 'content-type: application/json' http://127.0.0.1:3030/
```

### getblockhash

Returns the block hash of a block at the given block height in the best valid chain.

#### Arguments

|    Parameter   |  Type  | Required |                  Description                 |
|:-------------- |:------:|:--------:|:-------------------------------------------- |
| `block_height` | string |    Yes   | The block height of the requested block hash |

#### Response

| Parameter |  Type  |                      Description                      |
|:---------:|:------:|:-----------------------------------------------------:|
| `result`  | string | The block hash of the block at the given block height |

#### Example
```
curl --data-binary '{"jsonrpc": "2.0", "id":"documentation", "method": "getblockhash", "params": [100] }' -H 'content-type: application/json' http://127.0.0.1:3030/
```

### getrawtransaction

Returns hex encoded bytes of a transaction from its transaction id.

#### Arguments

|     Parameter    |  Type  | Required |                     Description                     |
|:---------------- |:------:|:--------:|:--------------------------------------------------- |
| `transaction_id` | string |    Yes   | The transaction id of the requested transaction hex |

#### Response

| Parameter |  Type  |            Description            |
|:---------:|:------:|:---------------------------------:|
| `result`  | string | The hex-encoded transaction bytes |

#### Example
```
curl --data-binary '{"jsonrpc": "2.0", "id":"documentation", "method": "getrawtransaction", "params": ["83fc73b8a104d7cdabe514ec4ddfeb7fd6284ff8e0a757d25d8479ed0ffe608b"] }' -H 'content-type: application/json' http://127.0.0.1:3030/
```

### gettransactioninfo

Returns information about a transaction from a transaction id.

#### Arguments


|     Parameter    |  Type  | Required |                      Description                     |
|:---------------- |:------:|:--------:|:---------------------------------------------------- |
| `transaction_id` | string |    Yes   | The transaction id of the requested transaction info |

#### Response

|        Parameter        |  Type  |                Description                |
|:-----------------------:|:------:|:----------------------------------------- |
| `txid`                  | string | The transaction id                        |
| `size`                  | number | The size of the transaction in bytes      |
| `old_serial_numbers`    | array  | The list of old record serial numbers     |
| `new_commitments`       | array  | The list of new record commitments        |
| `memo`                  | string | The transaction memo                      |
| `digest`                | string | The merkle tree digest                    |
| `transaction_proof`     | string | The transaction zero knowledge proof      |
| `predicate_commitment`  | string | The predicate verification key commitment |
| `local_data_commitment` | string | The local data commitment                 |
| `value balance`         | number | The transaction value balance             |
| `signatures`            | array  | The list of transaction signatures        |
| `transaction_metadata`  | object | The transaction metadata                  |

#### Example
```
curl --data-binary '{"jsonrpc": "2.0", "id":"documentation", "method": "gettransactioninfo", "params": ["83fc73b8a104d7cdabe514ec4ddfeb7fd6284ff8e0a757d25d8479ed0ffe608b"] }' -H 'content-type: application/json' http://127.0.0.1:3030/
```

### decoderawtransaction

Returns information about a transaction from serialized transaction bytes.

#### Arguments

|      Parameter      |  Type  | Required |            Description            |
|:------------------- |:------:|:--------:|:--------------------------------- |
| `transaction_bytes` | string |    Yes   | The raw transaction hex to decode |

#### Response

|        Parameter        |  Type  |                Description                |
|:-----------------------:|:------:|:----------------------------------------- |
| `txid`                  | string | The transaction id                        |
| `size`                  | number | The size of the transaction in bytes      |
| `old_serial_numbers`    | array  | The list of old record serial numbers     |
| `new_commitments`       | array  | The list of new record commitments        |
| `memo`                  | string | The transaction memo                      |
| `digest`                | string | The merkle tree digest                    |
| `transaction_proof`     | string | The transaction zero knowledge proof      |
| `predicate_commitment`  | string | The predicate verification key commitment |
| `local_data_commitment` | string | The local data commitment                 |
| `value balance`         | number | The transaction value balance             |
| `signatures`            | array  | The list of transaction signatures        |
| `transaction_metadata`  | object | The transaction metadata                  |

#### Example
```
curl --data-binary '{"jsonrpc": "2.0", "id":"documentation", "method": "decoderawtransaction", "params": ["transaction_hexstring"] }' -H 'content-type: application/json' http://127.0.0.1:3030/
```

### sendtransaction

Send raw transaction bytes to this node to be added into the mempool. If valid, the transaction will be stored and propagated to all peers.

#### Arguments

|      Parameter      |  Type  | Required |              Description             |
|:------------------- |:------:|:--------:|:------------------------------------ |
| `transaction_bytes` | string |    Yes   | The raw transaction hex to broadcast |

#### Response

| Parameter |  Type  |                 Description                |
|:---------:|:------:|:------------------------------------------ |
| `result`  | string | The transaction id of the sent transaction |

#### Example
```
curl --data-binary '{"jsonrpc": "2.0", "id":"documentation", "method": "sendtransaction", "params": ["transaction_hexstring"] }' -H 'content-type: application/json' http://127.0.0.1:3030/
```

### validaterawtransaction

Validate and return if the transaction is valid.

#### Arguments

|      Parameter      |  Type  | Required |             Description             |
|:------------------- |:------:|:--------:|:----------------------------------- |
| `transaction_bytes` | string |    Yes   | The raw transaction hex to validate |

#### Response

| Parameter |   Type  |             Description             |
|:---------:|:-------:|:----------------------------------- |
| `result`  | boolean | Check that the transaction is valid |

#### Example
```
curl --data-binary '{"jsonrpc": "2.0", "id":"documentation", "method": "validaterawtransaction", "params": ["transaction_hexstring"] }' -H 'content-type: application/json' http://127.0.0.1:3030/
```

### getconnectioncount

Returns the number of connected peers this node has.

#### Arguments

None

#### Response

| Parameter |  Type  |          Description          |
|:---------:|:------:|:----------------------------- |
| `result`  | number | The number of connected nodes |

#### Example
```
curl --data-binary '{"jsonrpc": "2.0", "id":"documentation", "method": "getconnectioncount", "params": [] }' -H 'content-type: application/json' http://127.0.0.1:3030/
```

### getpeerinfo

Returns the node's connected peers.

#### Arguments

None 

#### Response

| Parameter |  Type |           Description          |
|:---------:|:-----:|:------------------------------:|
| `peers`   | array | The list of connected peer IPs |

#### Example
```
curl --data-binary '{"jsonrpc": "2.0", "id":"documentation", "method": "getpeerinfo", "params": [] }' -H 'content-type: application/json' http://127.0.0.1:3030/
```

### getblocktemplate

Returns the current mempool and consensus information known by this node.

#### Arguments

None

#### Response

|       Parameter       |  Type  |                      Description                      |
|:--------------------- |:------:|:----------------------------------------------------- |
| `previous_block_hash` | string | The hash of current highest block                     |
| `block_height`        | number | The height of the next block                          |
| `time`                | number | The current timestamp                                 |
| `difficulty_target`   | number | The block difficulty target                           |
| `transactions`        | array  | The list of raw transactions to include in the block  |
| `coinbase_value`      | number | The amount spendable by the coinbase transaction      |

#### Example
```
curl --data-binary '{"jsonrpc": "2.0", "id":"documentation", "method": "getblocktemplate", "params": [] }' -H 'content-type: application/json' http://127.0.0.1:3030/
```

### decoderecord

Returns information about a record from serialized record hex.

#### Arguments

|    Parameter   |  Type  | Required |          Description         |
|:--------------:|:------:|:--------:|:----------------------------:|
| `record_bytes` | string |    Yes   | The raw record hex to decode |

#### Response

|        Parameter        |  Type  |             Description            |
|:----------------------- |:------:|:---------------------------------- |
| `account_public_key`    | string | The hash of current highest block  |
| `is_dummy`              | number | The height of the next block       |
| `value`                 | number | The current timestamp              |
| `payload`               | object | The record payload                 |
| `birth_predicate_repr`  | string | The birth predicate representation |
| `death_predicate_repr`  | string | The death predicate representation |
| `serial_number_nonce`   | string | The serial number nonce            |
| `commitment`            | string | The record commitment              |
| `commitment_randomness` | string | The record commitment randomness   |

#### Example
```
curl --data-binary '{"jsonrpc": "2.0", "id":"documentation", "method": "decoderecord", "params": ["record_hexstring"] }' -H 'content-type: application/json' http://127.0.0.1:3030/
```

## Protected Methods

These RPC methods are protected and will require authentication if `-rpc-username` and `-rpc-password` are set.

### createrawtransaction

#### Arguments

|          Parameter         |  Type  | Required |                        Description                       |
|:-------------------------- |:------:|:--------:|:-------------------------------------------------------- |
| `old_records`              |  array |    Yes   | An array of hex encoded records to be spent              |
| `old_account_private_keys` |  array |    Yes   | An array of private keys authorized to spend the records |
| `recipients`               |  array |    Yes   | The array of transaction recipient objects               |
| `memo`                     | string |    No    | The transaction memo                                     |
| `network_id`               | number |    Yes   | The network id of the transaction                        |

Transaction Recipient Object:

| Parameter |  Type  |            Description           |
|:---------:|:------:|:--------------------------------:|
| `address` | string | The recipient address            |
| `value`   | number | The amount sent to the recipient |

#### Response

|       Parameter       |  Type  |                  Description                  |
|:---------------------:|:------:|:--------------------------------------------- |
| `encoded_transaction` | string | The hex encoding of the generated transaction |
| `encoded_records`     | array  | The hex encodings of the generated records    |

#### Example
```
curl --user username:password --data-binary '{ 
    "jsonrpc":"2.0",
    "id": "1",
    "method": "createrawtransaction",
    "params": [
       {
        "old_records": ["record_hexstring"],
        "old_account_private_keys": ["private_key_string"],
        "recipients": [{
                "address": "address_string",
                "amount": amount
        }],
        "auxiliary": "auxiliary_hexstring",
        "memo": "memo_hexstring",
        "network_id": 0
       }
    ]
}' -H 'content-type: application/json' http://127.0.0.1:3030/
```

### fetchrecordcommitments

Return the node's stored record commitments.

#### Arguments

None.

#### Response

| Parameter |  Type |             Description            |
|:---------:|:-----:|:---------------------------------- |
| `result`  | array | The list stored record commitments |

#### Example
```
curl --user username:password --data-binary '{"jsonrpc": "2.0", "id":"documentation", "method": "fetchrecordcommitments", "params": [] }' -H 'content-type: application/json' http://127.0.0.1:3030/ 
```

### getrawrecord

Returns hex encoded bytes of a record from its record commitment.

#### Arguments

|      Parameter      |  Type  | Required |      Description      |
|:-------------------:|:------:|:--------:|:--------------------- |
| `record_commitment` | string |    Yes   | The record commitment |

#### Response

| Parameter |  Type  |          Description         |
|:---------:|:------:|:---------------------------- |
| `result`  | string | The hex-encoded record bytes |

#### Example
```
curl --user username:password --data-binary '{"jsonrpc": "2.0", "id":"documentation", "method": "getrawrecord", "params": ["86be61d5f3bd795e31615d6834efefca01ad023d57c0383e2231e094bcabfc05"] }' -H 'content-type: application/json' http://127.0.0.1:3030/ 
```

### createaccount

Generate a new account private key and public key pair.

#### Arguments

None.

#### Response

|   Parameter   |  Type  |         Description         |
|:------------- |:------:|:--------------------------- |
| `private_key` | string | An Aleo account private key |
| `public_key`  | string | An Aleo account public key  |

#### Example
```
curl --user username:password --data-binary '{"jsonrpc": "2.0", "id":"documentation", "method": "createaccount", "params": [] }' -H 'content-type: application/json' http://127.0.0.1:3030/ 
```
