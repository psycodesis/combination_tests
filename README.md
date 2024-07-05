<!--
 Copyright (c) 2024 Mariusz Zacirka
 
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Motivation

Sometimes you want to cover a relatiely big example space with the same testing code by parameterizing it. For the cases when the expected result can be inferred from the input parameters (e.g. by using alternative calculation) or it is enough to check some set of desired conditions (invariants) this crate will come in handy.

# Overview

`combination_tests` crate contains a rust macro `test_permutations` that generates test cases for a number of variables each assigned a set of its possible values. In other words the macro permutates given sets of values and generates a test case for each permutation.
E.g.: given a variable `a` with a set {1, 2, 3} and a variable `b` with a set {10, 20} the following permutation will be generated:
```rust
let a = 1; let b = 10;
let a = 1; let b = 20;
let a = 2; let b = 10;
let a = 2; let b = 20;
let a = 3; let b = 10;
let a = 3; let b = 20;
```
However the given values need to be identifiers since the macro internally uses them to construct modules. The variables and its sets need to be constructed using the usual `let` construct with all possible values separated with `or`.
The previous example would need all the values defined as constants, e.g.:
```rust
const A1: i32 = 1;
const A2: i32 = 2;
const A3: i32 = 3;
const B10: i32 = 10;
const B20: i32 = 20;
```
Using the above the variables would be defined as follows:
```rust
let a = A1 or A2 or A3;
let b = B10 or B20;
```
Each permutation needs some way to run tested code and check its result. This is realized with `when` and `then` clauses. Both can use the defined variables, e.g.: `a` and `b`.

The `when` clause is ment to run the tested code. The block of code needs to return a value which will be assigned to a given identifier. E.g.:
```rust
when actual_result = {
    let c = a + b;
    tested_function(c)
}
```
In the above example the result of running the `tested_function` will be assigned to the `actual_result`.

The `then` clause is used for checking the result of the `when` block, e.g. by asserting the result variable has got expected value. E.g.:
```rust
then {
    let expected_result = calc_expected_result(a, b);
    assert_eq!(expected_result, actual_result);
}
```

Putting the example together and adding a title `example_test`:
```rust
mod tests {
    use combination_tests::*;
    const A1: i32 = 1;
    const A2: i32 = 2;
    const A3: i32 = 3;
    const B10: i32 = 10;
    const B20: i32 = 20;

    test_permutations! {
        title example_test;
        let a = A1 or A2 or A3;
        let b = B10 or B20;
        when actual_result = {
            let c = a + b;
            tested_function(c)
        }
        then {
            let expected_result = calc_expected_result(a, b);
            assert_eq!(expected_result, actual_result);
        }
    }
}
```

# Usage