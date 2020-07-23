---
id: tests
title: Writing Tests
---

Use the `test` keyword to define tests in a leo program. Tests must have zero function inputs and zero function returns.

```leo
function main(a: u32) -> u32 {
    return a
}

test function expect_pass() {
    let a = 1u32;

    let res = main(a);

    assert_eq!(res, 1u32);
}

test function expect_fail() {
    assert_eq!(1u8, 0u8);
}
```
To run tests, use the `leo test` CLI [command](../../aleo/cli/develop#leo-test).

## Assert Equals
This macro will enforce that the two values are equal in the constraint system.

```leo
function main() {
    assert_eq!(45u32, 45u32);
  
    assert_eq!(2field, 2field);
  
    assert_eq!(true, true);
}
```

## The Anatomy of a Test Function
Inside the Leo `test function` signature you have access to all `imports`, `circuits`, and `functions` in the current scope.
```leo title="src/main.leo"
function add_one(a: u32) -> u32 {
    return a + 1;
}

test function test_add_one() {
    let one = 1u32;
    let two = 2u32;

    let res = add_one(one);
    
    assert_eq!(two, res);
}
```

In `test_add_one` we initialize **allocated** values `one` and `two`.
We provide `one` as an input to the `add_one` function and save the result in `res`.
In a real program execution, we would expect `one` to be provided in a Leo input `.in` file.

The last line of the test asserts that the type and value of `two` is equal to result `res`.

Next, run the test with
```bash
leo test
```

```leo title="console output:"
  leo  Running 1 tests
  leo  test tmp::test_add_one passed. Constraint system satisfied.
```
**Success!**

The console output clearly states that our test passed, and our constraint system is satisfied.

### Failing Tests
```leo title="src/main.leo"
function add_one(a: u32) -> u32 {
    return a + 1;
}

test function test_add_one() {
    let one = 1u32;
    let two = 2u32;

    let res = add_one(one);
    
    assert_eq!(one, res); // <- changed to `one`
}
```

Change the last line of `test_add_one` to assert that `one` is equal to the result `res`.

Run the test with
```bash
leo test
```

```leo title="console output:"
  leo  Running 1 tests
  leo  test tmp::test_add_one failed. Constraint system not satisfied.
```

As expected, the test now fails. The console output tells us that the constraint system is `not satified`.

Under the hood, the compiler executes the test in two parts. First, the test function is compiled to check for syntax
errors. Second, the circuit is run. Since test functions do not have input values, we can simply check to see if the circuit's
constraint system is satisfied instead of generating and verifying a full proof.

### Failing Test Compilation 

Tests with invalid syntax will fail before their circuit is run.

```leo title="src/main.leo"
function add_one(a: u32) -> u32 {
    return a + 1;
}

test function test_add_one() {
    let one = 1u32;
    let two = 2u32;

    let res = add_one(one, one); // <- added `one` as input
    
    assert_eq!(one, res);
}
```

Add a second `one` as input to the function call to `add_one`.

```leo title="console output:"
  leo  Running 1 tests
  leo  test tmp::test_add_one errored:    -->  2:1
                      |
                    2 |  function add_one(a: u32) -> u32 {
                      |
                      |
                      = function expected 1 inputs, found 2 inputs
```

As expected, the test fails telling us that we incorrectly provided 2 inputs to the `add_one` function.
Since we failed before running the circuit, there is no output about the constraint system.