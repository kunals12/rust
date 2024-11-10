## Overview of Rust Iterators
The iterator pattern allows you to perform some task on a sequence of items in turn. An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished. When you use iterators, you don’t have to reimplement that logic yourself.

In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up. For example, below code creates an iterator over the items in the vector v1 by calling the iter method defined on Vec<T>. This code by itself doesn’t do anything useful.

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();
```


### Key Concepts

- **Basic Iterators**: Access elements one-by-one in a sequence.
- **Iterator Adaptors**: Transform or filter elements while retaining laziness (they don’t compute values until needed).
- **IntoIterator**: Consumes a collection, transferring ownership of each item.
- **Double-ended Iterators**: Allows traversal from both ends of a collection.


## 1. Basic Iterator Example
A basic iterator provides simple element access through the .next() method. Here’s a snippet:

```rust
let numbers = vec![1, 2, 3, 4, 5];
let mut iter = numbers.iter();

while let Some(number) = iter.next() {
    println!("Got: {}", number);
}
```


## 2. Iterator Adaptors Example
Iterator adaptors can transform items, applying functions lazily (i.e., only when needed):
```rust
let numbers = vec![1, 2, 3, 4, 5];
let squares: Vec<_> = numbers.iter().map(|&x| x * x).collect();
println!("Squares: {:?}", squares);
```


## 3. IntoIterator Example
IntoIterator takes ownership of each item in a collection, making it useful when you don't need the original collection:
```rust
let numbers = vec![10, 20, 30];
for number in numbers.into_iter() {
    println!("IntoIterator gave: {}", number);
}
```
The for syntax when applied directly on the collection uses into_iter under the hood

## 4. Double Ended Iterator
Double-ended iterators support traversing from both ends:

```rust
let numbers = vec![1, 2, 3, 4, 5];
let mut iter = numbers.iter();

while let Some(front) = iter.next() {
    println!("Front: {}", front);
}
while let Some(back) = iter.next_back() {
    println!("Back: {}", back);
}
```


# Iter
If you want immutable references to the inner variables and don’t want to transfer ownership

# IterMut
If you want mutable references to the inner variables and don’t want to transfer ownership

# IterInto
If you want to move the variable into the iterator and don’t want to use it afterwards