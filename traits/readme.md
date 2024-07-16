### Introduction to Traits

Traits in Rust are a way to define shared behavior across multiple types. They allow you to write generic and reusable code. A trait defines a set of methods that a type must implement.

`Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.`


### Simple Explanation

Think of traits as a way to define common behavior that different types should have. For example, if you have multiple shapes like Square and Rectangle, you can define a trait Shape that requires an area method. This way, you can guarantee that all shapes will have an area method, regardless of their specific implementation.

```rust
// Define a struct 'Square' with fields for side length, line width, and color
struct Square {
    side: f32,
    line_width: u8,
    color: String,
}

// Define a struct 'Rectangle' with fields for length, width, line width, and color
struct Rectangle {
    length: f32,
    width: f32,
    line_width: u8,
    color: String,
}
```

## Define Trait
```rust
// Define a trait 'Shape' with a method 'area' that returns a float
trait Shape {
    fn area(&self) -> f32;
}
```

The Shape trait defines a single method area that calculates the area of the shape.

## Implementing the Trait
```rust
// Implement the 'Shape' trait for 'Square'
impl Shape for Square {
    fn area(&self) -> f32 {
        let area_of_square = self.side * self.side;
        println!("Square area {}", area_of_square);
        area_of_square
    }
}

// Implement the 'Shape' trait for 'Rectangle'
impl Shape for Rectangle {
    fn area(&self) -> f32 {
        let area_of_rectangle = self.length * self.width;
        println!("Rectangle area {}", area_of_rectangle);
        area_of_rectangle
    }
}
```

These implementations provide the specific logic for calculating the area of a Square and a Rectangle.

## main function
```rust
fn main() {
    // Create a new 'Square' instance
    let s1 = Square::new(5.3, 1, String::from("Red"));

    // Create a new 'Rectangle' instance
    let r1 = Rectangle::new(5.3, 4.3, 1, String::from("Blue"));

    // Calculate and print the area of the square
    s1.area();

    // Calculate and print the area of the rectangle
    r1.area();
}
```


### TRAITS BOUNDS
Trait bounds in Rust specify that a generic type parameter must implement a particular trait. They allow you to enforce constraints on generic types, ensuring that they provide certain behavior or methods.

## How It Works
When you use a generic type in a function, struct, or trait, you can use trait bounds to specify that the generic type must implement one or more traits. This ensures that the methods defined by those traits are available for that type.

## Syntax
Trait bounds are specified using the `:` symbol. Here’s the basic syntax:

```rust
fn function_name<T: TraitName>(param: T) {
    // function body
}
```
You can also specify multiple trait bounds:
```rust
fn function_name<T: Trait1 + Trait2>(param: T) {
    // function body
}
```
