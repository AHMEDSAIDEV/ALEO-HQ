---
title: Lexical Structure
sidebar_label: Lexical Structure
---

## Comments
Leo supports in **line comments** using:

`//`

This will comment all text on the rest of this line.

Leo supports **block comments** using:

`/* ... */`

This will comment out all text inside `...`. Nested comments are not supported.


## Identifiers

Identifiers in Leo must start with an alphabetical character.
The first character can be followed by any number of alphanumerical or `_` characters.
An identifier that is equal to a keyword will result in a syntax error.
Examples of identifiers include:

* variables
* functions
* function parameters
* circuits
* circuit members

## Keywords 

The following Leo keywords can only be used in their correct contexts.
They cannot be used as the names of identifiers.

```leo
address
as
const
else
false
function
for
if
import
in
let
mut
return
self
Self
static
test
true
```

