# Rust Testing Example

This repository provides a simple example of how to write and run tests in Rust. Testing is an essential part of software development, and Rust makes it easy to write and run tests for your code.

## Getting Started

To get started with testing in Rust, you need to have Rust installed on your machine. You can install Rust using `rustup` by following the instructions on the [official Rust website](https://doc.rust-lang.org/book/ch11-01-writing-tests.html).

## Writing Tests

In Rust, tests are written inside the `tests` module, using the `#[cfg(test)]` attribute to indicate that the module should only be included when running tests. Each test function is annotated with the `#[test]` attribute.

Here's a simple example:

```rust
// src/lib.rs

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-2, -3), -5);
    }
}
```

In this example, the add function is tested with two test cases: one for positive numbers and one for negative numbers.

### Running Tests
To run the tests, use the following command:


```terminal
cargo test
```

This command will compile your code (if necessary) and run all tests defined in your project. The output will indicate which tests passed and which tests failed.

```terminal
$ cargo test
   Compiling rust_testing_example v0.1.0 (file:///path/to/your/project)
    Finished test [unoptimized + debuginfo] target(s) in 1.34s
     Running target/debug/deps/rust_testing_example-abc123def456

running 2 tests
test tests::test_add ... ok
test tests::test_add_negative ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

```

## Testing Macros
Rust provides several macros to help with writing tests. Here are some commonly used ones:

## `#[cfg(test)]`
This attribute is used to include the test module only when running tests. It ensures that the test code is not included in the final binary when you build your project for release.

## `#[test]`
This attribute is used to mark a function as a test function. The test runner will execute all functions annotated with #[test].

## `assert!`
This macro is used to assert that a condition is true. If the condition is false, the test will fail.
```rust
#[test]
fn test_condition() {
    assert!(1 + 1 == 2);
}
```
## `assert_eq! and assert_ne!`
These macros are used to assert that two expressions are equal (assert_eq!) or not equal (assert_ne!). If the assertion fails, the test will fail and provide a message showing the values of the expressions.

```rust
#[test]
fn test_equality() {
    assert_eq!(add(2, 3), 5);
}

#[test]
fn test_inequality() {
    assert_ne!(add(2, 2), 5);
}
```

## `assert!, assert_eq!, and assert_ne! with Custom Messages`
These macros can also take an optional custom message that will be displayed if the assertion fails.

```rust
#[test]
fn test_with_custom_message() {
    assert_eq!(add(2, 2), 4, "Addition result is incorrect");
}
```