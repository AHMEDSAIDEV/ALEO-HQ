---
id: posw_predicate
title: The PoSW Predicate
sidebar_label: The PoSW Predicate
---

The predicate used as part of the PoSW circuit verifies the inclusion of transactions in a given block.
The various building blocks are defined below alongside the relevant implementation parameters.

## System State

The state of the system is given by a Merkle tree `Tree_G(h)` of depth `h` over a CRT function `G: {0, 1}^k -> {0, 1}^(k/2)`,
where `G` is taken to be SHA-256 with `k = 512`. We denote this as the "state tree".
Each leaf is the unique ID of a transaction to be processed,
and the variable `state` is the root of the tree.

<!--

<img align="left" src="Binary_tree.png" style="float:right"></img>

-->

The PoSW circuit takes the `q <= d` subtree of the state tree and computes a Merkle tree of depth `q`.
The leaves of the tree are the depth `q` elements of the state tree `Tree_H(h)`,
instantiated over `k`-bit leaves with a different CRT function `H: {0, 1}^k -> {0, 1}^(k/2)` as a new PoSW tree `Tree_H(q)`.
This layout is illustrated in the diagram on the left.
For example, for `G = BLS12_377` we set `H` as the `512`-bit Pedersen CRH with output truncated to `256` bits. 

The circuit implementation for `H` masks the witness variables based on a pseudorandom seed,
which is part of the predicate statement.
This is required to achieve non-amortization guarantees.
We set `q = 3` throughout.

## Pedersen Primitives

The `k`-bit Pedersen hash function over `G` is a CRT hash given by:
```
H(G, x) = ∏_{for i = 1..k} G_i^{x_i}
```
where `G_i` in `G` are randomly sampled generators and `x_i` the `i`-th input bit of `x`.
CRT security of this function reduces to the hardness of the Discrete Logarithm Problem (DLP)
over the group `G`.

The above function can be evaluated in a 'masked' fashion, using the primitives below.

### Symmetric Pedersen Gadget

The $k$-bit symmetric Pedersen hash is defined with the same security guarantees as `H: {0, 1}^k -> G` where:
```
H_sym(H, rho) = ∏_{for i = 1..k} H_i^{1 - 2 * rho_i}
```

#### Circuit Structure

Define group variables `Q = (Q_x, Q_y)`, `h_i = (h^i_x, h^i_y)` in `(F_p^2)^k`. Check the following evaluations:
- If `rho_i = 0`, set `h_i = H_i`, else if `rho_i = 1` set to `h_i = H_i^{-1}`.
- `Q_0` is the identity and `Q_i = Q_{i-1} * h_i`.

This requires `k` Edwards multiplications (6 constraints each),
and a bit lookup for each of the `h_i` in addition to `k` booleanity checks.

This is evaluated by ``precomputed_base_symmetric_multiscalar_mul`` in ``PedersenCRHGadget``.

### Masked Pedersen Gadget

The `k`-length masked Pedersen hash function over `G` is a CRT hash function `H_mask: {0, 1}^k x {0, 1}^k x G -> G` given by:
```
H_mask^{G, H}(rho, x, P) = P * ∏_{for i = 1..k} (1[x_i (+) rho_i = 1] G_i^{2 * x_i - 1} H_i^{2 * rho_i - 1} + 1[x_i (+) rho_i = 0] H_i^{2 * rho_i - 1})
```

where `x_i` and `rho_i` the `i`-th bits of `x` and `rho` respectively,
while `G_i` in `G` are randomly sampled generators of `G` and `(+)` the bitwise XOR operation.
The variable `P` in `G` is appended as an input as well, for the demasking operation.

#### Circuit Structure

Define group variables `Q = (Q_x, Q_y)`, `g_i = (g^i_x, g^i_y)` in `(F_p^2)^k` and
boolean variables `z` in `F_p^k`. Perform the following evaluations:
- With a `2`-bit lookup, for all `i` in `[k]` set `g_i` :=
 	- `H_i^{-1}` if `rho_i = 0` and `x_i = 0`
 	- `H_i` if $\rho_i = 1$ and $x_i = 1$
 	- `G_i * H_i^{-1}` if `rho_i = 1` and `x_i = 0`
 	- `G_i^{-1} * H_i` if `rho_i = 0` and `x_i = 1`
- `Q_0 = P` and `Q_i = Q_{i-1} * g_i`.

This requires `k` Edwards multiplications (6 constraints each), a `2`-bit lookup for each of the `g_i` (2 constraints each) and `k` booleanity checks.

This is evaluated by ``precomputed_base_scalar_mul_masked`` in ``PedersenCRHGadget``.

### Pedersen Hash Gadget

We instantiate a circuit verifying `M` evaluations of `H^G` using circuits for `H_mask^{G,H}` and
`H_sym^H` over `G`. Note that elements are variables in `F_p`,
while pairs of variables `(z_x,z_y)` in `F_p^2` are parsed as elliptic curve points in
`G`. We presume that the `H_i, G_i` in `G` have been precomputed and are accessible as constants.

#### Inputs

The `k`-length masked evaluation of `M` Pedersen hashes takes as inputs:
- For `i` in `[ M ]`, boolean variables `x^i = {x^i_1, .., x^i_k}`. 
- A boolean seed `rho` in `{0, 1}^k` that is a subset of `F^k_p`.

#### Evaluations

- Set `z <- H_sym^{H}(rho)`.
- For all `i` in `[M]`,  set `(o^x_i, o_i^y) = H_mask^{G,H}(rho, x^i, z)`.

#### Outputs

The `k/2` length set of variables `{o^1_x, ..., o^k_x}` in `(F_p)^k` as the truncated outputs.

### Instantiation

We use BLS12-377 as the underlying group,
which implies an output length of `256+1 = 257` bits (using point-compression) which we truncate to `256` bits.
Security reduction to the hardness of ECDLP yields a security level of `lambda ~= 128 bits`.
The input length is set to `k = 512 bits`.

## PoSW Circuit 

The PoSW tree `Tree_H(q)` takes in the subroots of the state tree's `q`-depth nodes as leaves,
and uses the `k`-bit Pedersen hash gadget with respect to a seed parameter `rho` to compute the root `state_i`.
The seed parameter `rho = PRF(state_i || n)` is the output of a pseudorandom function PRF with boolean inputs,
the nonce `n` and the tree root.

### Seed Generation
We generate the seed `rho` in the following way for each predicate:

1. Given input nonce `n` in `{0, 1}^256` and `state_i` in `{0, 1}^256`, compute `rho_0` in `{0, 1}^256` as
`rho_0 = BLAKE(n || state_i)`, where `||` represents string concatenation.

2. If the `i`-th bit `rho_{0, i}` of `rho_0` is `0` or `1`, set the `(2i-1)`-th and `2i`-th bits of `rho`
as `10` or `01` respectively. This gives a `rho` in `{0, 1}^512` of constant Hamming distance `256`.

This is all done outside of the circuit, and is required input format for every valid instance.

### Circuit Size

#### Statement-Witness Definition

A valid statement `phi = <state_i, n>` in `{0, 1}^{512}` as a subset of `F_p^512`, where: 

1. `state_i` in `{0, 1}^256` the bitwise representation of the PoSW root node of the updated state variable.
2. `n` in `{0, 1}^256` the bitwise representation of the nonce.

The witness `w` for the above statement consists of:

1. A boolean representation of `rho` in `{0, 1}^512`.

2. The subroot leaves `{x_i}_{i = 1}^{2^q}`, `x_i` in `{0, 1}^512` corresponding to `state_i`.

3. Boolean representations of the intermediate node values of `Tree_H(q)`.

#### Evaluations

For the root `state_i` and all internal nodes of `Tree_H(q)`,
perform a computation of the `H` gadget with the node value as output and its children as inputs. 

The PoSW circuit verifies that `Tree_H(q)` is correctly generated.
This requires the computation of `2^{q-1} + 1` instances of `H`.
