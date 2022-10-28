---
id: resources
title: Developer Resources
---

## 1. The Aleo Workshop Repository

📜 A starter guide to build applications on Aleo 📜

https://github.com/AleoHQ/workshop

## 2. The Awesome Aleo Repository

🏎️ A curated list of Aleo & Leo code and resources 🏎️

https://github.com/howardwu/awesome-aleo

## 3. The Discord Community

💬 Share what you are building in the #aleo-language channel 💬

https://discord.gg/aleohq

# Style Guide


This guide is provided to point developers in the right direction when writing Leo code.
There are many conventions that are unique to Leo language and the circuits it generates.

This guide is a living document.
As new Leo programming conventions arise and old ones become obsolete this guide should reflect the changes.
Feel free to add your comments and recommendations [here](#contributing).


## Code Layout

### Indentation
4 spaces per indentation level.

### Blank lines

A single blank line should separate `structs`s and `function`s.
Multiple imports can be optionally separated by a single blank line.
The last import at the top of the file should be followed by a blank line.

```leo title="Yes:"
import std.io.Write;
import std.math.Add;

struct A {
    // ...
}

function foo() {
    // ...
}

@test
function test_foo() {
    // ...
}
```

```leo title="No:"
import std.io.Write;


import std.math.Add;
struct A {
    // ...
}
function foo() {
    // ...
}

@test
function test_foo() {
    // ...
}
```
### Naming Conventions

| Item                | Convention                          |
|---------------------|-------------------------------------|
| Packages            | kebab-case (but prefer single word) |
| Circuits            | CamelCase                           |
| Circuit Members     | snake_case                          |
| Functions           | snake_case                          |
| Function Parameters | snake_case                          |
| Variables           | snake_case                          |
| Inputs              | snake_case                          |
| Tests               | snake_case                          |

### Circuit Definitions

* Circuits should have value members defined above function members and be separated by a single blank line.
* Multiple value members should be comma separated and have their own line.
* Static functions should be defined before non-static functions.

```leo
struct A {
    x: u32,
    y: u32,

    function new() {
        // ...
    }

    function foo(self) {
        // ...
    }
}
```

### Layout
Leo file elements should be ordered:
1. Imports
2. Program declaration
3. Mappings
4. Circuits + Structs 
5. Functions
6. Tests

### Braces
Opening braces always go on the same line.
```leo
struct A {
    // ...
}

function main() {
    // ...
}

let a = A { };
```
### Semicolons
Every statement including the `return` statement should end in a semicolon.
```leo
let a = 1u32;
let mut b = a + 5;
b *= 2;

return b
```

### Commas
Trailing commas should be included whenever the closing delimiter appears on a separate line.
```leo 
let a = A { x: 0, y: 1 };

let a = A {
    x: 0,
    y: 1,
};
```

# Common Patterns

Building off of the style guide, here is a list of common patterns that a Leo developer may encounter
as well as the recommended code solution.

## Conditional Branches

Conditional `if else` statements in Leo are expensive. It is preferred to use ternary `? :` expressions.

```leo title="Example:"
if (condition) {
    return a
} else {
    return b
} 
```

```leo title="Alternative:"
return condition ? a : b
```

### Why?
Ternary expressions are the cheapest form of conditional.
We can resolve the *first expression* and *second expression* values before evaluating the *condition*.
This is very easy to convert into a circuit because we know that each expression does not depend on information in later statements.

In the original `Example`,
We cannot resolve the return statements before evaluating the condition.
As a solution, Leo creates branches in the circuit so both paths can be evaluated.

```leo title="branch 1, condition = true"
return a
```

```leo title="branch 2, condition = false"
return b
```
When the input value `condition` is fetched at proving time, we select a branch of the circuit to evaluate.
Observe that the statement `return a` is repeated in both branches.
The cost of every computation within the conditional will be doubled.
This greatly increases the constraint numbers and slows down the circuit.

# Contributing

Thank you for helping make Leo better!

Before contributing, please view the [Contributor Code of Conduct](https://github.com/AleoHQ/leo/blob/master/CONTRIBUTING.md).
By participating in this project - In the issues, pull requests, or Gitter channels -
you agree to abide by the terms.

## Report an Issue

To report an issue, please use the [GitHub issues tracker](https://github.com/AleoHQ/leo/issues). When reporting issues, please mention the following details:

- Which version of Leo you are using.
- What was the source code (if applicable).
- Which platform are you running on.
- How to reproduce the issue.
- What was the result of the issue.
- What the expected behavior is.

Reducing the source code that caused the issue to a bare minimum is always very helpful and sometimes clarifies a misunderstanding.

## Make a Pull Request

Start by forking off of the `testnet3` branch to make your changes. Commit messages should clearly explain why and what you changed.

If you need to pull in any changes from the `testnet3` branch after making your fork (for example, to resolve potential merge conflicts),
please avoid using git merge and instead, git rebase your branch. Rebasing will help us review your changes easily.

### Tools Required

To build Leo from source you will need the following tools:
- The latest rust stable version and nightly version.
    - Recommend that you install multiple versions using `rustup`.
- Cargo
    - Rusty Hook install via `cargo install rusty-hook`.
- Clippy
    - Via rustup, if you didn't do the default rustup install `rustup componenet add clippy`.

### Formatting

Please do the following before opening a PR.
- `cargo +nightly fmt --all` will format all your code.
- `cargo clippy --all-features --examples --all --benches`

### Tests

If your code adds new functionality, please write tests to confirm the new features function as expected. Refer to existing tests for examples of how tests are expected to be written. Please read refer to the [parser tests section](#parser-tests). To run the tests please use the following command `cargo test --all --features ci_skip --no-fail-fast`.

#### **Parser Tests**

In the root directory of the repository, there is a "tests" directory.
To add a parser test, look at the Example Leo files in the parser sub-directory.
Then when running the test command, make sure you have the environment variable `CLEAR_LEO_TEST_EXPECTATIONS` set to true. For example, on a UNIX environment, you could run the following command `CLEAR_LEO_TEST_EXPECTATIONS=true cargo test --all --features ci_skip --no-fail-fast`.

### Grammar

In the root directory of the repository, there exists a "docs/grammar" directory. In that directory, there is an "abnf-grammar.txt" file that has the grammar rules in ABNF format. If your changes affect a grammar rule, we may ask you to modify it in that txt file. After you do so, make sure to go into the directory and run `cargo run > README.md`. Doing so will ensure that the README file for the grammar is up to date.

We appreciate your hard work!