### Introduction to Traits

Traits in Rust are a way to define shared behavior across multiple types. They allow you to write generic and reusable code. A trait defines a set of methods that a type must implement.

`Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.`


### Simple Explanation

Think of traits as a way to define common behavior that different types should have. For example, if you have multiple shapes like Square and Rectangle, you can define a trait Shape that requires an area method. This way, you can guarantee that all shapes will have an area method, regardless of their specific implementation.

## Define Trait
```rust
// Define a trait 'Shape' with a method 'area' that returns a float
trait Shape {
    fn area(&self) -> f32;
}

```
