// Define a struct named `Fruit` with two fields: `apples` and `bananas`
struct Fruit {
    apples: i32,
    bananas: i32,
}

// Define a function `increase_fruit` that takes a `Fruit` struct and returns a new `Fruit` struct
// with the number of apples doubled and the number of bananas tripled
fn increase_fruit(fruit: Fruit) -> Fruit {
    Fruit {
        apples: fruit.apples * 2,
        bananas: fruit.bananas * 3,
    }
}

// Define a function `new_fruit` that returns a `Fruit` struct with a fixed number of apples and bananas
fn new_fruit() -> Fruit {
    Fruit {
        apples: 10,
        bananas: 5,
    }
}

// Define a function `print_fruit` that takes a `Fruit` struct and prints the number of apples and bananas
fn print_fruit(fruit: Fruit) {
    println!(
        "You have {} apples and {} bananas",
        fruit.apples, fruit.bananas
    );
}

// The main function is the entry point of the program
fn main() {
    // Create a new `Fruit` struct using `new_fruit`
    // Pass the `Fruit` struct to `increase_fruit` to get a new `Fruit` struct with increased values
    // Pass the new `Fruit` struct to `print_fruit` to print the number of apples and bananas
    print_fruit(increase_fruit(new_fruit()));
}
