# Understanding `HashMap` in Rust

## Introduction

In Rust, a `HashMap` is a collection of key-value pairs where each key is unique. It is one of the most commonly used data structures for storing and managing data that can be quickly accessed by a key.

## Creating a `HashMap`

To create a `HashMap`, you need to import the `HashMap` struct from the `std::collections` module. You can create an empty `HashMap` and then insert key-value pairs into it.

## Inserting Values

You can insert key-value pairs into a `HashMap` using the `insert` method. The `insert` method takes two arguments: the key and the value you want to insert.

## Accessing Values

To access a value associated with a specific key, you can use the `get` method. The `get` method returns an `Option` type, which can be `Some(&value)` if the key exists or `None` if the key does not exist. You can use pattern matching or the `unwrap` method to handle the result.

## Checking for Keys

To check if a `HashMap` contains a specific key, you can use the `contains_key` method. This method returns a boolean value indicating whether the key is present in the `HashMap`.

## Iterating Over a `HashMap`

You can iterate over all key-value pairs in a `HashMap` using a `for` loop. When iterating, you get references to the keys and values, so you need to use the appropriate reference types in the loop.

## Example Usage

Here is an example demonstrating the typical operations you can perform with a `HashMap`:

```rust
use std::collections::HashMap;

fn main() {
    // Create a new mutable HashMap to store people's names and ages
    let mut person: HashMap<&str, u32> = HashMap::new();

    // Insert some key-value pairs into the HashMap
    person.insert("Nouman", 32);
    person.insert("Kunal", 23);

    // Get the value associated with the key "Kunal" and print it
    println!("The age is {:?}", person.get("Kunal").unwrap());

    // Check if the key "Nouman" exists in the HashMap and print a message
    if person.contains_key("Nouman") {
        println!("Key exists");
    } else {
        println!("Key doesn't exist");
    }

    // Use a match expression to check if the key "Kunal" exists and print a message
    match person.get("Kunal") {
        Some(_) => println!("Value exists"),
        None => println!("Not exists"),
    }

    // Iterate over the HashMap and print each key-value pair
    for (name, age) in &person {
        println!("The person {} has an age of {}", name, age);
    }
}
