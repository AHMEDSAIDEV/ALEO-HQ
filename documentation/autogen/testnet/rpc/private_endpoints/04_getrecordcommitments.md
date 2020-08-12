---
id: getrecordcommitments
title: getrecordcommitments
sidebar_label: getrecordcommitments
---

Returns a list of record commitments that are stored on the full node.

### Protected Endpoint

Yes

### Arguments

`None`

### Response

| Parameter |  Type |              Description              |
|:---------:|:-----:|:------------------------------------- |
| `result`  | array | The list of stored record commitments |

### Example
```ignore
curl --user username:password --data-binary '{"jsonrpc": "2.0", "id":"documentation", "method": "getrecordcommitments", "params": [] }' -H 'content-type: application/json' http://127.0.0.1:3030/ 
```
