# 1. Ownership 
### What is Ownership
In Rust, every value has a single owner: the variable that controls the memory for that value.

When the owner goes out of scope, the value is automatically deallocated.

### Ownership Rules
1. Each value in Rust has a single owner.

2. A value can only have one owner at a time.

3. When the owner goes out of scope, Rust drops (frees) the value automatically.

```rust
fn main() {
    let s1 = String::from("Hello"); // `s1` owns the String
    let s2 = s1; // Ownership moves to `s2`

    // println!("{}", s1); // Error: `s1` is no longer valid
    println!("{}", s2); // Works: `s2` is the new owner
}
```


# 2. Borrowing and References
### What is Borrowing
Borrowing allows a value to be accessed without taking ownership. It’s achieved using references.


### Types of references

### 1. Immutable References (&T):
Multiple immutable references are allowed.

```rust
fn main() {
    let s = String::from("Hello");
    let r1 = &s; // Immutable reference
    let r2 = &s; // Another immutable reference

    println!("{} and {}", r1, r2); // Both references can be used
}
```


### Mutable References (&mut T):
Only one mutable reference is allowed at a time.

```rust
fn main() {
    let mut s = String::from("Hello");
    let r1 = &mut s; // Mutable reference
    // let r2 = &mut s; // Error: cannot borrow `s` as mutable more than once

    println!("{}", r1); // `r1` is used
}
```

### Borrowing Rules
1. You can have multiple immutable references or one mutable reference but not both at the same time.

2. References must always be valid (no dangling references).


# 3. Combining Ownership and Borrowing
```rust
fn main() {
    let s = String::from("Hello");
    let len = calculate_length(&s); // Pass reference, ownership is not moved
    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: &String) -> usize {
    s.len() // Use the borrowed reference
}
```

# 4. Dangling References
Rust prevents dangling references by ensuring that references do not outlive the data they point to.

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x; // Error: `x` does not live long enough
    }
    println!("{}", r); // `r` would point to invalid memory
}
```


# 5. Slices and Borrowing
String slices (&str) and array slices (&[T]) are references to part of a collection.
```rust
fn main() {
    let s = String::from("Hello, world");
    let hello = &s[0..5]; // Slice borrowing part of the string
    let world = &s[7..12];

    println!("{} {}", hello, world);
}
```

