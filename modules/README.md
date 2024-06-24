# Introduction

In Rust, the code organization is achieved through modules, crates, and packages. These concepts allow developers to structure and manage their code efficiently, promoting reusability and modularity.

## Modules

A module is a collection of items: functions, structs, traits, impl blocks, and even other modules.

Modules in Rust are a way to organize code within a crate. They allow you to group related functions, structs, enums, and other items together. Modules can be nested, and they help in encapsulating and organizing the code in a logical manner.

### Basic Example

``` Rust 
// Define a module named `math`
mod math {
    // A public function within the `math` module
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    // A nested module named `geometry`
    pub mod geometry {
        // A public struct within the `geometry` module
        pub struct Rectangle {
            pub width: f64,
            pub height: f64,
        }

        impl Rectangle {
            // A public method to calculate the area of the rectangle
            pub fn area(&self) -> f64 {
                self.width * self.height
            }
        }
    }
}

// Using items from the `math` module and its nested module `geometry`
fn main() {
    // Using the `add` function from the `math` module
    let sum = math::add(3, 4);
    println!("Sum: {}", sum);

    // Using the `Rectangle` struct from the nested `geometry` module
    let rect = math::geometry::Rectangle { width: 5.0, height: 4.0 };
    println!("Area: {}", rect.area());
}
```


## Crates

A crate is a compilation unit in Rust. It can be a binary crate or a library crate:

`Binary crate`: Produces an executable file. It has a main function as the entry point.

`Library crate`: Produces a library that can be used by other programs or libraries. It does not have a main function.

Crates are the primary unit of code distribution in Rust and are managed by Cargo, Rust's package manager and build system.


## Packages

A package is a collection of one or more crates that are built, tested, and published together. A package contains a Cargo.toml file that describes how to build those crates. Every package must contain at least one crate.

### Example of Cargo.toml

``` Rust 
[package]
name = "my_package"
version = "0.1.0"
authors = ["Author Name <author@example.com>"]
edition = "2021"

[dependencies]
serde = "1.0"
```