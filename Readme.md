Galvanic-assert: Matcher-based assertions for easier testing
============================================================
This crate provides a new assertion macro `assert_that!` based on **matching predicates** (matchers) to
 * make **writing** asserts easier
 * make **reading** asserts comprehendable
 * easily **extend** the assertion framework
 * provide a large list **common matchers**
 * integrate with **galvanic-test** and **galvanic-mock**
 * be used with your favourite test framework

The 2-minute tutorial
---------------------
Each assertion has the form `assert_that!(SOME_VALUE, MATCHES_SOMETHING);`.
To check if some value satisfies some matching predicate, e.g., `less_than`, `contains_in_order`, `is_variant!`, ...; we can write something like:
```rust
#[macro_use]
extern crate galvanic_assert;
use galvanic_assert::matchers::*;

#[test]
fn expression_should_compute_correct_value {
    assert_that!(1+2, eq(3));
}
```

or ...
```rust
use galvanic_assert::matchers::collection::*;
#[test]
fn expression_should_compute_correct_value {
    let some_values = vec![5,1,3,4,2];
    assert_that!(some_values, contains_in_any_order(vec![1,2,3,4,5]));
}
```

or ...
```rust
enum Variants {
    First(i32),
    Second { x: i32, y: i32 },
    Third
}

#[test]
fn should_be_the_correct_variant {
    let var = Second { x: 1, y: 2 };
    assert_that!(var, is_variant!(Variants::Second));
}
```

It is also possible to combine multiple matchers to create more expressive ones ...
```rust
#[test]
fn expression_should_compute_correct_value {
    assert_that!(1+2, all_of!(greater_than(0), less_than(5));
    assert_that!(1+2, any_of!(greater_than(5), less_than(5));
}
```

If this is not enough you can write your own matchers, either as a closure ...
```rust
#[test]
fn expression_should_compute_correct_value {
    assert_that!(1+2, |x| {
        let builder = MatchResultBuilder::for_("odd");
        if x % 2 == 1 { builder.matched() } else { builder.failed_because("result is not odd") }
    });
}
```

or if it is easy enough, as an expression ...
```rust
#[test]
fn expression_should_compute_correct_value {
    let val = 1 + 2;
    assert_that!(val % 2 == 1, otherwise "result is not odd");
}
```
If it is more complex implement the `Matcher` trait for some struct representing the state of the matcher.

Asserting positive things is good, but sometimes we expect that something goes horribly wrong ...
```rust
fn press_big_red_button() {
    panic!("shouldn't have done that ...");
}

#[test]
fn someting_should_go_wrong {
    assert_that!(press_big_red_button(), panics);
}
```

... except when it doesn't.
```rust
fn press_faulty_red_button() {
    return;
    panic!("shouldn't have done that ...");
}

#[test]
fn everything_is_fine {
    assert_that!(press_faulty_red_button(), does not panic);
}
```

Not much more to say---have a look at the documentation and the growing list of matchers.
If something is missing or broken, please open an issue or a pull request.

And remember ...
*only tested code is happy code!*